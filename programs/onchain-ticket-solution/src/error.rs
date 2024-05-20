use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
	#[msg("This ticket has already been sold.")]
	TicketAlreadySold,
	#[msg("The sale has not started yet.")]
	SaleNotStarted,
	#[msg("Insufficient funds to purchase the ticket.")]
	InsufficientFunds,
}