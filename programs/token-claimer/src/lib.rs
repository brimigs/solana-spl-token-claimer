use anchor_lang::prelude::*;

declare_id!("kfyRBhWw3Q2rusi3fJtnNamYLtgMvFoq9naWH3b1KVh");

#[program]
pub mod token_claimer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
