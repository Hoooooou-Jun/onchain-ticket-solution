use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
pub mod error;
pub mod instructions;
use instructions::*;

declare_id!("92z53zQ9AuDC1s7xjgmiLfaPRpdkvPKsgFQK95V6MY7k");

#[program]
pub mod onchain_ticket_solution {
	use super::*;

	pub fn init_event(
		ctx: Context<InitEvent>,
		name: String,
		ticket_open_date: i64,
	) -> Result<()> {
		init_event::init_event(ctx, name, ticket_open_date)
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
		mint_ticket::mint_ticket(ctx, name, seat_number, img_cid, event_key, event_date, price)
	}

	pub fn purchase_ticket(
		ctx: Context<PurchaseTicket>,
		name: String,
		seat_number: String,
	) -> Result<()> {
		purchase_ticket::purchase_ticket(ctx, name, seat_number)
	}

	pub fn refund_ticket(
		ctx: Context<RefundTicket>,
		name: String,
		seat_number: String,
	) -> Result<()> {
		refund_ticket::refund_ticket(ctx, name, seat_number)
	}
}
