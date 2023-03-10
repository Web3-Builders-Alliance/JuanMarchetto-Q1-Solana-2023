use anchor_lang::{prelude::*, system_program};
use anchor_spl::dex::serum_dex::{
    instruction::SelfTradeBehavior,
    matching::{OrderType, Side},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    dex::{self, cancel_order_v2, close_open_orders, new_order_v3, CancelOrderV2, Dex, NewOrderV3},
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, MetadataAccount,
    },
    token::{
        initialize_mint2, InitializeMint2, Mint, Token, TokenAccount, Transfer as SplTransfer,
    },
};
use std::num::NonZeroU64;
use whitelist;

use mpl_token_metadata::state::DataV2;

declare_id!("7YKyo13HtdB823RiWHacDR74wc7VeU8vkMZGJDP2nSUB");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    //methods for depositing and withdrawing native tokens
    pub fn deposit_native(ctx: Context<DepositNative>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let sys_program = &ctx.accounts.system_program;

        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn withdraw_native(ctx: Context<WithdrawNative>, amount: u64) -> Result<()> {
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

    //methods for depositing and withdrawing fungible SPL tokens
    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
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
        let deposit_account = &ctx.accounts.deposit_account;

        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.pda_auth.to_account_info(),
        };

        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    // want to create a limit order.
    // casts NewOrder instruction to dex::NewOrderV3
    pub fn new_order(ctx: Context<NewOrder>, limit_price: NonZeroU64) -> Result<()> {
        let dex_program = ctx.accounts.dex_program.to_account_info();
        let side = Side::Ask;
        let max_coin_qty = NonZeroU64::new(10000).unwrap();
        let max_native_pc_qty_including_fees = NonZeroU64::new(10000).unwrap();
        let self_trade_behavior = SelfTradeBehavior::DecrementTake;
        let order_type = OrderType::Limit;
        let client_order_id = 0;
        let limit = 0;

        let order: NewOrderV3 = ctx.accounts.into();

        let cpi = CpiContext::new(dex_program, order);

        new_order_v3(
            cpi,
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            self_trade_behavior,
            order_type,
            client_order_id,
            limit,
        )
    }

    /// CODING CHALLENGE: complete this instruction handler
    /// pass in the variables needed to cancel and order,
    /// replace "......" with the correct variables
    pub fn cancel_order(ctx: Context<CancelOrder>, order_id: u128) -> Result<()> {
        let dex_program = ctx.accounts.dex_program.to_account_info();
        let order: CancelOrderV2 = ctx.accounts.into();
        let cpi = CpiContext::new(dex_program, order);
        cancel_order_v2(cpi, Side::Ask, order_id)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = deposit_auth, space = DepositBase::LEN)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositNative<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(seeds = [
        pda_auth.key.as_ref(),
        deposit_auth.key().as_ref(),],
        bump,
        seeds::program = whitelist_program)
    ]
    pub whithelist: Account<'info, whitelist::WhitelistPDA>,
    pub whitelist_program: Program<'info, whitelist::program::Whitelist>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawNative<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(seeds = [
        pda_auth.key.as_ref(),
        deposit_auth.key().as_ref(),],
        bump,
        seeds::program = whitelist_program)
    ]
    pub whithelist: Account<'info, whitelist::WhitelistPDA>,
    pub whitelist_program: Program<'info, whitelist::program::Whitelist>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
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
    #[account(seeds = [
        pda_auth.key.as_ref(),
        deposit_auth.key().as_ref(),],
        bump,
        seeds::program = whitelist_program)
    ]
    pub whithelist: Account<'info, whitelist::WhitelistPDA>,
    pub whitelist_program: Program<'info, whitelist::program::Whitelist>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(mut)]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut,
        associated_token::mint = token_mint,
        associated_token::authority = pda_auth,
    )]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(seeds = [
        pda_auth.key.as_ref(),
        deposit_auth.key().as_ref(),],
        bump,
        seeds::program = whitelist_program)
    ]
    pub whithelist: Account<'info, whitelist::WhitelistPDA>,
    pub whitelist_program: Program<'info, whitelist::program::Whitelist>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewOrder<'info> {
    /// CHECK: no need to check this.
    pub market: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub open_orders: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub request_queue: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub event_queue: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub market_bids: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub market_asks: AccountInfo<'info>,
    // Token account where funds are transferred from for the order. If
    // posting a bid market A/B, then this is the SPL token account for B.
    /// CHECK: no need to check this.
    pub order_payer_token_account: Account<'info, TokenAccount>,
    /// CHECK: no need to check this.
    pub open_orders_authority: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    /// CHECK: no need to check this.
    pub coin_vault: Account<'info, TokenAccount>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    /// CHECK: no need to check this.
    pub pc_vault: Account<'info, TokenAccount>,
    /// CHECK: no need to check this.
    pub token_program: Program<'info, Token>,
    /// CHECK: no need to check this.
    pub rent: Sysvar<'info, Rent>,
    pub dex_program: Program<'info, Dex>,
}

impl<'info> From<&mut NewOrder<'info>> for NewOrderV3<'info> {
    fn from(new_order: &mut NewOrder<'info>) -> Self {
        NewOrderV3 {
            market: new_order.market.clone(),
            open_orders: new_order.open_orders.clone(),
            request_queue: new_order.request_queue.clone(),
            event_queue: new_order.event_queue.clone(),
            market_bids: new_order.market_bids.clone(),
            market_asks: new_order.market_asks.clone(),
            order_payer_token_account: new_order
                .order_payer_token_account
                .to_account_info()
                .clone(),
            open_orders_authority: new_order.open_orders_authority.clone(),
            coin_vault: new_order.coin_vault.to_account_info().clone(),
            pc_vault: new_order.pc_vault.to_account_info().clone(),
            token_program: new_order.token_program.to_account_info().clone(),
            rent: new_order.rent.to_account_info().clone(),
        }
    }
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    /// CHECK: no need to check this.
    pub market: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub open_orders: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub request_queue: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub event_queue: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub market_bids: AccountInfo<'info>,
    /// CHECK: no need to check this.
    pub market_asks: AccountInfo<'info>,
    // Token account where funds are transferred from for the order. If
    // posting a bid market A/B, then this is the SPL token account for B.
    /// CHECK: no need to check this.
    pub order_payer_token_account: Account<'info, TokenAccount>,
    /// CHECK: no need to check this.
    pub open_orders_authority: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    /// CHECK: no need to check this.
    pub coin_vault: Account<'info, TokenAccount>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    /// CHECK: no need to check this.
    pub pc_vault: Account<'info, TokenAccount>,
    /// CHECK: no need to check this.
    pub token_program: Program<'info, Token>,
    /// CHECK: no need to check this.
    pub rent: Sysvar<'info, Rent>,
    pub dex_program: Program<'info, Dex>,
}

impl<'info> From<&mut CancelOrder<'info>> for CancelOrderV2<'info> {
    fn from(new_order: &mut CancelOrder<'info>) -> Self {
        CancelOrderV2 {
            market: new_order.market.clone(),
            open_orders: new_order.open_orders.clone(),
            event_queue: new_order.event_queue.clone(),
            market_bids: new_order.market_bids.clone(),
            market_asks: new_order.market_asks.clone(),
            open_orders_authority: new_order.open_orders_authority.clone(),
        }
    }
}

#[derive(Accounts)]
pub struct RemoveLimit {}

#[derive(Accounts)]
pub struct AcceptLimit {}

#[derive(Accounts)]
pub struct MintftAndCreateMetadata<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(init, payer = deposit_auth, space = MetadataAccount::LEN)]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    /// CHECK: add constraints later
    pub edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct DepositBase {
    pub deposit_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
}

impl DepositBase {
    const LEN: usize = 8 + 32 + 1 + 1 + 1;
}

#[account]
pub struct Limit {
    pub asset_holding_pda: Option<Pubkey>,
    pub asset: Asset,
    pub ask_price_per_asset: u64,
    pub ask_asset: Asset,
    pub ask_asset_pda: Option<Pubkey>,
}

#[account]
pub struct Asset {
    pub asset_type: String,
    pub asset_metadata: Option<Pubkey>,
    pub asset_mint: Option<Pubkey>,
}

const OPTION_PUBKEY_LEN: usize = 1 + 32;

impl Limit {
    const LEN: usize = OPTION_PUBKEY_LEN * 2 + Asset::LEN * 2 + 8;
}

impl Asset {
    const LEN: usize = 32 + OPTION_PUBKEY_LEN * 2;
}

pub fn mint_nft_and_create_metadata<'info>(
    ctx: Context<MintftAndCreateMetadata<'info>>,
    mint: &AccountInfo<'info>,
    mint_auth: &AccountInfo<'info>,
    data: DataV2,
) -> Result<()> {
    //let mint = create_mint()?;
    let mint_cpi = InitializeMint2 {
        mint: mint.to_account_info(),
    };
    let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_cpi);

    initialize_mint2(cpi, 0, mint_auth.key, Some(mint_auth.key))?;

    let meta_data_cpi = CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata.to_account_info(),
        mint: mint.to_account_info(),
        mint_authority: mint.to_account_info(),
        payer: mint.to_account_info(),
        update_authority: mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), meta_data_cpi);

    let data = DataV2 {
        name: "Placeholder".to_string(),
        symbol: "PLC".to_string(),
        uri: "localhost".to_string(),
        seller_fee_basis_points: 100,
        creators: None,
        collection: None,
        uses: None,
    };

    let meta_out = create_metadata_accounts_v3(cpi, data, true, true, None)?;

    /*let master_edition_cpi = CreateMasterEditionV3 {
        metadata: ctx.accounts.metadata.to_account_info(),
        edition: ctx.accounts.edition.to_account_info(),
        mint: mint.to_account_info(),
        mint_authority: mint.to_account_info(),
        payer: mint.to_account_info(),
        update_authority: mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };
    let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), meta_data_cpi);


    anchor_spl::metadata::create_master_edition_v3(master_edition_cpi, Some(1))?;*/
    Ok(())
}
