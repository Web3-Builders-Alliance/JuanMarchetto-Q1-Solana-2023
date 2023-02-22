use anchor_lang::{prelude::*, system_program};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer as SplTransfer},
};

declare_id!("94syforMJhHfxQUkSNqTq6aUrkaJwW17BbaSFTfXe67j");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _name: String) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    pub fn deposit_into_pda(ctx: Context<DepositInto>, amount: u64) -> Result<()> {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
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
        let sys_program = &ctx.accounts.system_program;
        let deposit_account = &ctx.accounts.deposit_account;
        let pda_auth = &mut ctx.accounts.pda_auth;
        let sol_vault = &mut ctx.accounts.sol_vault;

        let cpi_accounts = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.deposit_auth.to_account_info(),
        };

        let seeds = &[
            b"sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.sol_vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.deposit_auth.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
      

        Ok(())
    }


}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(init, payer = deposit_auth, space = Vault::LEN )]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
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
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        payer = deposit_auth,
        associated_token::authority = pda_auth,
    )]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSpl {
}

#[account]
pub struct Vault {
    pub deposit_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User isn't the owner of this vault")]
    InvalidOwner,
}


impl Vault {
    const LEN: usize = 8 + 32 + 1 + 1 + 1;
}