use anchor_lang::prelude::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.auth = *ctx.accounts.initializer.key;
        Ok(())
    }

    pub fn AddToWhitelist(ctx: Context<AddWhitelist>, new_user: Pubkey) -> Result<()> {
        if ctx.accounts.signer.key != &ctx.accounts.whitelist.auth {
            return Err(error!(ErrorCode::NotTheAuth));
        }
        Ok(())
    }

    pub fn RemoveFromWhitelist(ctx: Context<RemoveWhitelist>) -> Result<()> {
        if ctx.accounts.signer.key != &ctx.accounts.whitelist.auth {
            return Err(error!(ErrorCode::NotTheAuth));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 8)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_user: Pubkey,)]
pub struct AddWhitelist<'info> {
    #[account(
        init,
        payer = signer,
        space = 8,
        seeds = [signer.key.as_ref(), new_user.key().as_ref(),],
        bump
    )]
    pub WhitelistPDA: Account<'info, WhitelistPDA>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
    #[account(
        mut,
        close = signer,
    )]
    pub WhitelistPDA: Account<'info, WhitelistPDA>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Whitelist {
    pub auth: Pubkey
}

#[account]
pub struct WhitelistPDA{} 

#[error_code]
pub enum ErrorCode {
    #[msg("User isn't the authority")]
    NotTheAuth,
}