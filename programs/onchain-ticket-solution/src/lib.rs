use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use std::mem::size_of;

pub mod constant;
pub mod states;
pub mod error;
use crate::{constant::*, states::*};

declare_id!("92z53zQ9AuDC1s7xjgmiLfaPRpdkvPKsgFQK95V6MY7k");

#[program]
pub mod onchain_ticket_solution {
	use super::*;

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

	pub fn mint_ticket(
		ctx: Context<MintTicket>,
		name: String,
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

	pub fn purchase_ticket(
		ctx: Context<PurchaseTicket>,
		name: String,
		seat_number: String,
	) -> Result<()> {
		let ticket = &mut ctx.accounts.ticket;
		let ticket_authority = &ctx.accounts.ticket_authority;
		let buyer = &ctx.accounts.buyer;

		if ticket.is_sold {
			return Err(error!(error::ErrorCode::TicketAlreadySold));
		}

		if Clock::get()?.unix_timestamp < ticket.event_date {
			return Err(error!(error::ErrorCode::SaleNotStarted));
		}

		if buyer.lamports() < ticket.price {
			return Err(error!(error::ErrorCode::InsufficientFunds))
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

		ticket.is_sold = true;
		ticket.authority = *buyer.key;

		Ok(())
	}
}

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