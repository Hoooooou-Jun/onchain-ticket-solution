import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainTicketSolution } from "../target/types/onchain_ticket_solution";
import { BN } from "bn.js";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

const unixTimestampConverter = (data) => {
  let timestamp = new Date(Number(data) * 1000)
  return timestamp.getUTCFullYear() + '-' +
  ('0' + (timestamp.getUTCMonth() + 1)).slice(-2) + '-' +
  ('0' + timestamp.getUTCDate()).slice(-2) + ' ' +
  ('0' + timestamp.getUTCHours()).slice(-2) + ':' +
  ('0' + timestamp.getUTCMinutes()).slice(-2);
}

const unixTimestampSerialize = (date) => {
  const eventDateBigInt = BigInt(date); // BigInt로 변환 (음수 처리를 위해)
  const buffer = Buffer.alloc(8); // Buffer를 생성하고 8바이트의 공간을 할당 (i64는 8바이트 크기)
  buffer.writeBigInt64LE(eventDateBigInt, 0); // Buffer에 BigInt 값을 little-endian 형식으로 쓰기
  return buffer;
}

describe("onchain-ticket-solution", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.OnchainTicketSolution as Program<OnchainTicketSolution>;

  const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('event'),
      Buffer.from('장범준 콘서트'),
    ],
    program.programId
  );

  it("initialize event", async () => {
    /* 한국 시간은 UTC+9 */
    const date = new Date('2024-05-25T10:00:00Z'); // UTC 시간
    const unixTimestamp = new BN(Math.floor(date.getTime() / 1000));

    const tx = await program.methods
      .initEvent(
        "장범준 콘서트",
        unixTimestamp,
      ).accounts({
        event: eventPda
      })
      .rpc();

    const account = await program.account.eventAccount.all();
    console.log("Your transaction signature", tx);
    console.log(account);

    const resultTime = unixTimestampConverter(account[0].account.ticketOpenDate.toString());
    console.log(resultTime);
  });

  it("mint ticket", async () => {
    const eventDate = 1716631200;
    const bufferDate = unixTimestampSerialize(eventDate);

    const [ticketPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('ticket'),
        eventPda.toBuffer(),
        Buffer.from('R1'),
        bufferDate
      ],
      program.programId,
    );

    const tx = await program.methods.mintTicket(
      "장범준 콘서트",
      "R1",
      "dummycid",
      eventPda,
      new BN(1716631200),
      new BN(1 * 1e9),
    ).accounts({
      event: eventPda,
      ticket: ticketPda,
    })
    .rpc()

    const account = await program.account.ticketAccount.all();
    console.log(account);
  });

  it("purchase ticket", async () => {
    const purchaseUser = anchor.web3.Keypair.generate()
    const airdropSignature = await provider.connection.requestAirdrop(
      purchaseUser.publicKey,
      15 * anchor.web3.LAMPORTS_PER_SOL
    )
    await provider.connection.confirmTransaction(airdropSignature);
    const purchaseUserBalance = await provider.connection.getBalance(purchaseUser.publicKey);
    const balance = await provider.connection.getBalance(provider.wallet.publicKey);
    console.log("ticket purchaser : ", purchaseUser.publicKey.toString(), purchaseUserBalance / anchor.web3.LAMPORTS_PER_SOL);
    console.log("authority : ", balance / anchor.web3.LAMPORTS_PER_SOL);

    const eventDate = 1716631200;
    const bufferDate = unixTimestampSerialize(eventDate);

    const [ticketPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('ticket'),
        eventPda.toBuffer(),
        Buffer.from('R1'),
        bufferDate
      ],
      program.programId,
    );

    try {
      const tx = await program.methods.purchaseTicket(
        "장범준 콘서트",
        "R1",
      )
      .accounts({
        event: eventPda,
        ticket: ticketPda,
        buyer: purchaseUser.publicKey,
        ticketAuthority: provider.wallet.publicKey
      })
      .signers([purchaseUser])
      .rpc()
    } catch(err) {
      console.log(err);
    }

    const account = await program.account.ticketAccount.all();
    console.log(account);

    console.log(await provider.connection.getBalance(purchaseUser.publicKey) / anchor.web3.LAMPORTS_PER_SOL)
    console.log(await provider.connection.getBalance(provider.wallet.publicKey) / anchor.web3.LAMPORTS_PER_SOL)
  });
});
