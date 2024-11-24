use anchor_lang::prelude::*;

declare_id!("GfrRXsiBDhcXR29h7DjqMzRokUu6bnZf3YaVfbyUqYD1");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
