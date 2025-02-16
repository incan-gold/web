use anchor_lang::prelude::*;

declare_id!("5C1ZZhbTN2snUwu9vBTLEJoCyw3QdBPDdsesEkpXnWJX");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
