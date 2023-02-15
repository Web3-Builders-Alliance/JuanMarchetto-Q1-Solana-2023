use anchor_lang::{
    prelude::*,
    solana_program::{self, program::invoke, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer as SplTransfer},
};

declare_id!("94syforMJhHfxQUkSNqTq6aUrkaJwW17BbaSFTfXe67j");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _name: String) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn deposit_into_pda(ctx: Context<DepositInto>, amount: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(
                ctx.accounts.payer.key,
                &ctx.accounts.vault.key(),
                amount,
            ),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.vault.to_account_info().clone(),
            ],
        )?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user = &mut ctx.accounts.user;

        if vault.owner != *user.key {
            return Err(error!(ErrorCode::InvalidOwner));
        }

        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(
                ctx.accounts.payer.key,
                &ctx.accounts.vault.key(),
                amount,
            ),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.vault.to_account_info().clone(),
            ],
        )?;
        Ok(())
    }

    /*pub fn withdraw_spl(ctx: Context<WithdrawSPL>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user = &mut ctx.accounts.user;

        if vault.owner != *user.key {
            return Err(error!(ErrorCode::InvalidOwner));
        }

        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.try_borrow_mut_lamports()? += amount;

        Ok(())
    }*/
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + solana_program::pubkey::PUBKEY_BYTES,
        seeds = [name.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    /// CHECK:
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositInto<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct DepositSPL<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub tokem_program: Program<'info, Token>,
//    pub user_token_account: Account<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User isn't the owner of this vault")]
    InvalidOwner,
}
