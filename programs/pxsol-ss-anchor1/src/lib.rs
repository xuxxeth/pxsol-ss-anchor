use anchor_lang::prelude::*;

declare_id!("8vGxkbSj5aynQ5gxEQ1zJ7XEWmnkKQuB7xb5Ce2QcD97");

#[program]
pub mod pxsol_ss_anchor1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
