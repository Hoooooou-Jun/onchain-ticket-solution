use anchor_lang::prelude::*;

declare_id!("92z53zQ9AuDC1s7xjgmiLfaPRpdkvPKsgFQK95V6MY7k");

#[program]
pub mod onchain_ticket_solution {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
