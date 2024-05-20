use anchor_lang::prelude::*;

use crate::{constant::*, states::*, error::ErrorCode::*};

#[derive(Accounts)]
#[instruction(
	name: String,
	seat_number: String,
)]
pub struct BurnTicket<'info> {
	#[account(
		mut,
		seeds = [
			EVENT_SEED,
			name.as_bytes(),
		],
		bump
	)]
	pub event: Account<'info, EventAccount>,
	#[account(
		mut,
		seeds = [
			TICKET_SEED,
			event.key().as_ref(),
			seat_number.as_bytes(),
			&event.ticket_open_date.to_le_bytes(),
		],
		bump,
		close=authority,
		constraint = ticket.authority == event.authority @ Unauthorized,
	)]
	pub ticket: Account<'info, TicketAccount>,
	#[account(mut)]
	pub authority: Signer<'info>,
	pub system_program: Program<'info, System>,
}

pub fn burn_ticket(
	ctx: Context<BurnTicket>,
	name: String,
	seat_number: String,
) -> Result<()> {
	let ticket = &mut ctx.accounts.ticket;

	msg!("Ticket for {} burned.", name);
	msg!("Seat Number : {}", seat_number);
	msg!("Event Date : {}", ticket.event_date);

	Ok(())
}