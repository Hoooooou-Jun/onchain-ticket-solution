use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

use crate::{constant::*, states::*, error::ErrorCode::*};

#[derive(Accounts)]
#[instruction(
	name: String,
	seat_number: String,
)]
pub struct PurchaseTicket<'info> {
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
		bump
	)]
	pub ticket: Account<'info, TicketAccount>,
	/// CHECK:
	#[account(
		mut,
	)]
	pub ticket_authority: AccountInfo<'info>,
	#[account(mut)]
	pub buyer: Signer<'info>,
	pub system_program: Program<'info, System>,
}

pub fn purchase_ticket(
	ctx: Context<PurchaseTicket>,
	_name: String,
	_seat_number: String,
) -> Result<()> {
	let ticket = &mut ctx.accounts.ticket;
	let ticket_authority = &ctx.accounts.ticket_authority;
	let buyer = &ctx.accounts.buyer;

	if matches!(ticket.state, TicketState::Unsealed) && matches!(ticket.state, TicketState::Used) {
		return Err(error!(TicketAlreadySold));
	}

	if Clock::get()?.unix_timestamp < ticket.event_date {
		return Err(error!(SaleNotStarted));
	}

	if buyer.lamports() < ticket.price {
		return Err(error!(InsufficientFunds))
	}

	invoke(
		&system_instruction::transfer(
			buyer.key,
			ticket_authority.key,
			ticket.price
		),
		&[
			buyer.to_account_info(),
			ticket_authority.to_account_info(),
			ctx.accounts.system_program.to_account_info(),
		],
	)?;
	ticket.state = TicketState::Unsealed;
	ticket.authority = *buyer.key;

	Ok(())
}