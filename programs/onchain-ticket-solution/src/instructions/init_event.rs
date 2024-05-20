use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constant::*, states::*};

#[derive(Accounts)]
#[instruction(
	name: String
)]
pub struct InitEvent<'info> {
	#[account(
		init,
		seeds = [
			EVENT_SEED,
			name.as_bytes(),
		],
		bump,
		payer = authority,
		space = size_of::<EventAccount>() + name.len()
	)]
	pub event: Account<'info, EventAccount>,
	#[account(mut)]
	pub authority: Signer<'info>,
	pub system_program: Program<'info, System>,
}

pub fn init_event(
	ctx: Context<InitEvent>,
	name: String,
	ticket_open_date: i64,
) -> Result<()> {
	let event = &mut ctx.accounts.event;
	event.authority = ctx.accounts.authority.key();
	event.name = name;
	event.ticket_open_date = ticket_open_date;
	event.ticket_count = 0;

	Ok(())
}