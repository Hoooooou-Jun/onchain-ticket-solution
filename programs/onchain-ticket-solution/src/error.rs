use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
	#[msg("This ticket has already been sold.")]
	TicketAlreadySold,
	#[msg("The sale has not started yet.")]
	SaleNotStarted,
	#[msg("Insufficient funds to purchase the ticket.")]
	InsufficientFunds,
	#[msg("This ticket is not sold")]
	TicketNotSold,
	#[msg("Unauthorized user")]
	Unauthorized,
}

// #[error_code]
// pub enum ProgramErrorCode {
//     #[msg("Invalid Mint account space")]
//     InvalidMintAccountSpace,
//     #[msg("Cant initialize metadata_pointer")]
//     CantInitializeMetadataPointer,
// }