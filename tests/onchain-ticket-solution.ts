import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainTicketSolution } from "../target/types/onchain_ticket_solution";
import { BN } from "bn.js";
import { unixTimestampConverter } from "./unixTimestampConverter";
import { unixTimestampSerialize } from "./unixTimestampSerialize";

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

  const purchaseUser = anchor.web3.Keypair.generate()

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

  it("refund ticket", async () => {
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

    const tx = await program.methods.refundTicket(
      "장범준 콘서트",
      "R1",
    ).accounts({
      event: eventPda,
      ticket: ticketPda,
      eventAuthority: provider.wallet.publicKey,
      buyer: purchaseUser.publicKey
    })
    .signers([purchaseUser])
    .rpc();

    const account = await program.account.ticketAccount.all();
    console.log(account);

    console.log(await provider.connection.getBalance(purchaseUser.publicKey) / anchor.web3.LAMPORTS_PER_SOL)
    console.log(await provider.connection.getBalance(provider.wallet.publicKey) / anchor.web3.LAMPORTS_PER_SOL)
  });
});
