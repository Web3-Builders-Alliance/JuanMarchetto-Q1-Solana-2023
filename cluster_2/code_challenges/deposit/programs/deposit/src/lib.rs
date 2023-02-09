use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction, self},
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
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.vault.key(),
                ctx.accounts.payer.key,
                amount,
            ),
            &[
                ctx.accounts.vault.to_account_info().clone(),
                ctx.accounts.payer.to_account_info().clone(),
            ],
        )?;
        Ok(())
    }
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
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey
}