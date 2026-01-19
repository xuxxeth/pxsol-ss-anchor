use anchor_lang::prelude::*;

declare_id!("ARs9xGKBkvhf9pm37pCvujuv1G4sNFX2Rt1943EHEGCk");

const SEED: &[u8] = b"pxsol-ss-anchor";

#[program]
pub mod pxsol_ss_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let account_user = &ctx.accounts.user;
        let user_pda = &mut ctx.accounts.user_pda;
        user_pda.auth = account_user.key();
        
        user_pda.bump = ctx.bumps.user_pda;
        user_pda.data = vec![];
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: Vec<u8>) -> Result<()> {
        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;

        require!(
            account_user_pda.auth == account_user.key(),
            CustomError::Unauthorized
        );
        account_user_pda.data = data;
        let data_str = String::from_utf8_lossy(&account_user_pda.data);
        msg!("Data updated successfully: {:?}", data_str);

        let account_user_pda_info = account_user_pda.to_account_info();
        let rent = Rent::get()?;
        let required_lamports = rent.minimum_balance(account_user_pda_info.data_len());
        let lamports_to_refund = account_user_pda_info.lamports()
            .saturating_sub(rent.minimum_balance(account_user_pda_info.data_len()));
        if lamports_to_refund > 0 {
            **account_user_pda_info.lamports.borrow_mut() -= lamports_to_refund;
            **account_user.to_account_info().lamports.borrow_mut() += lamports_to_refund;
        }


        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [SEED, user.key().as_ref()],
        bump,
        space = Data::space_for(64),
    )]
    pub user_pda: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_data: Vec<u8>)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [SEED, user.key().as_ref()],
        bump = user_pda.bump,
        realloc = Data::space_for(new_data.len()),
        realloc::payer = user,
        realloc::zero = false,
        constraint = user_pda.auth == user.key(),
    )]
    pub user_pda: Account<'info, Data>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Data {
    pub auth: Pubkey,
    pub bump: u8,
    pub data: Vec<u8>,
}
impl Data {
    pub fn space_for(data_len: usize) -> usize {
        8 + // discriminator
        32 + // auth
        1 + // bump
        4 + data_len // data vector
    }
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access to the account.")]
    Unauthorized,
}