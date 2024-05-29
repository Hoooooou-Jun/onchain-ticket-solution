use anchor_lang::prelude::*;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum TicketState {
    Sealed,
    Unsealed,
    Used,
}

#[account]
pub struct EventAccount {
	pub authority: Pubkey,
	pub name: String,
	pub ticket_open_date: i64,
	pub ticket_count: u32,
}

#[account]
pub struct TicketAccount {
	pub authority: Pubkey,
	pub event_key: Pubkey,
	pub event_date: i64,
	pub price: u64,
	pub img_cid: String,
	pub seat_number: String,
	pub state: TicketState,
}
