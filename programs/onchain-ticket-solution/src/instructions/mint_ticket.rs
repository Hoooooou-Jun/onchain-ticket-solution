use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constant::*, states::*};

#[derive(Accounts)]
#[instruction(
	name: String,
	seat_number: String,
	img_cid: String,
)]
pub struct MintTicket<'info> {
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
		init,
		seeds = [
			TICKET_SEED,
			event.key().as_ref(),
			seat_number.as_bytes(),
			&event.ticket_open_date.to_le_bytes(),
		],
		bump,
		payer = authority,
		space = size_of::<TicketAccount>() + name.len() + seat_number.len() + img_cid.len()
	)]
	pub ticket: Account<'info, TicketAccount>,
	#[account(mut)]
	pub authority: Signer<'info>,
	pub system_program: Program<'info, System>,
}

pub fn mint_ticket(
	ctx: Context<MintTicket>,
	_name: String,
	seat_number: String,
	img_cid: String,
	event_key: Pubkey,
	event_date: i64,
	price: u64,
) -> Result<()> {
	let event = &mut ctx.accounts.event;
	let ticket = &mut ctx.accounts.ticket;

	event.ticket_count += 1;
	ticket.authority = ctx.accounts.authority.key();
	ticket.event_key = event_key;
	ticket.event_date = event_date;
	ticket.price = price;
	ticket.img_cid = img_cid;
	ticket.seat_number = seat_number;

	Ok(())
}