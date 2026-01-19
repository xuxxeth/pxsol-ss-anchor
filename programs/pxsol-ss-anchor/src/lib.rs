use anchor_lang::prelude::*;

declare_id!("ARs9xGKBkvhf9pm37pCvujuv1G4sNFX2Rt1943EHEGCk");

#[program]
pub mod pxsol_ss_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
