use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        Ok(())
    }

    pub fn deposit_into_pda(ctx: Context<DepositInto>, amount_to_pda: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(
                ctx.accounts.payer.key,
                &ctx.accounts.pda.key(),
                amount_to_pda,
            ),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.pda.to_account_info().clone(),
            ],
        )?;
        Ok(())
    }

    pub fn withdaw(ctx: Context<DepositInto>, amount_to_pda: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.pda.key(),
                ctx.accounts.payer.key,
                amount_to_pda,
            ),
            &[
                ctx.accounts.pda.to_account_info().clone(),
                ctx.accounts.payer.to_account_info().clone(),

            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8,
        seeds = [b"deposit".as_ref(),],
        bump

    )]
    pub pda: Account<'info, DepositSpace>,
    /// CHECK:
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositInto<'info> {
    #[account(mut)]
    pub pda: Account<'info, DepositSpace>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositSpace {
}