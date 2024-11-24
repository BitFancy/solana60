use anchor_lang::prelude::*;

declare_id!("7DcNUJ6M2xqQEMwfaJ8VyLQJNw9nv3fospcPn1KRZLWv");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        msg!("You said {:?}", message);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
