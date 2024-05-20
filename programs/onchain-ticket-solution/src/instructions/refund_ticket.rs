use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

use crate::{constant::*, states::*, error::ErrorCode::*};

#[derive(Accounts)]
#[instruction(
	name: String,
	seat_number: String,
)]
pub struct RefundTicket<'info> {
	#[account(
		mut,
		seeds = [
			EVENT_SEED,
			name.as_bytes(),
		],
		bump,
	)]
	pub event: Account<'info, EventAccount>,
	#[account(
		mut,
		seeds=[
			TICKET_SEED,
			event.key().as_ref(),
			seat_number.as_bytes(),
			&event.ticket_open_date.to_le_bytes(),
		],
		bump,
	)]
	pub ticket: Account<'info, TicketAccount>,
	/// CHECK:
	#[account(
		mut,
	)]
	pub event_authority: AccountInfo<'info>,
	#[account(mut)]
	pub buyer: Signer<'info>,
	pub system_program: Program<'info, System>,
}

pub fn refund_ticket(
    ctx: Context<RefundTicket>,
    _name: String,
    _seat_number: String,
) -> Result<()> {
    let ticket = &mut ctx.accounts.ticket;
    let event_authority = &ctx.accounts.event_authority;
    let buyer = &ctx.accounts.buyer;

    if !ticket.is_sold {
        return Err(error!(TicketNotSold));
    }

    if ticket.authority != *buyer.key {
        return Err(error!(Unauthorized));
    }

    /* 환불 규정 날짜 이외 에러처리 필요 */

    invoke(
        &system_instruction::transfer(
            event_authority.key,
            buyer.key,
            ticket.price
        ),
        &[
            event_authority.to_account_info(),
            buyer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    ticket.is_sold = false;
    ticket.authority = *event_authority.key;

    Ok(())
}