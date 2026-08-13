use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::{
    SEED_CONFIG_ACCOUNT, SEED_LISTING_ACCOUNT, SEED_LISTING_TOKEN_VAULT_ACCOUNT, SEED_ORDER_ACCOUNT,
};
use crate::error::CustomError;
use crate::handlers::helpers::{
    settle_token_on_accept, settlement_fee_discount, SettlementFeeDiscountMode,
};

use crate::state::{require_fee_vault_account, Config, Listing, Order, OrderClosedReason};

pub fn process_payer_confirm_token_order(
    ctx: Context<PayerConfirmTokenOrder>,
    id: String,
) -> Result<()> {
    let config = &ctx.accounts.config_account;
    require_fee_vault_account(config, &ctx.accounts.fee_vault_account.to_account_info())?;

    require!(!ctx.accounts.order_account.closed, CustomError::ClosedError);

    require!(
        ctx.accounts.order_account.id == id,
        CustomError::PaymentIdMismatch
    );
    require_keys_eq!(
        ctx.accounts.order_account.order_token_vault_account,
        ctx.accounts.order_token_vault_account.key(),
        CustomError::TokenVaultMismatch
    );
    require_keys_eq!(
        ctx.accounts.order_account.mint_account,
        ctx.accounts.mint_account.key(),
        CustomError::MintAccountMismatch
    );
    require_keys_eq!(
        ctx.accounts.order_account.payer_token_account,
        ctx.accounts.payer_token_account.key(),
        CustomError::TokenAccountMismatch
    );
    require_keys_eq!(
        ctx.accounts.order_account.payee_token_account,
        ctx.accounts.payee_token_account.key(),
        CustomError::TokenAccountMismatch
    );
    require_keys_eq!(
        ctx.accounts.order_account.creator,
        ctx.accounts.payment_creator.key(),
        CustomError::PaymentOnlyCreator
    );

    let payer_info = ctx.accounts.payer.to_account_info();
    let payee_info = ctx.accounts.payee.to_account_info();
    let fee_vault_info = ctx.accounts.fee_vault_account.to_account_info();
    let token_vault_info = ctx.accounts.order_token_vault_account.to_account_info();
    let payer_token_info = ctx.accounts.payer_token_account.to_account_info();
    let payee_token_info = ctx.accounts.payee_token_account.to_account_info();
    let vault_token_info = ctx.accounts.vault_token_account.to_account_info();
    let mint_info = ctx.accounts.mint_account.to_account_info();
    let token_program_info = ctx.accounts.token_program.to_account_info();
    let payment_amount = ctx.accounts.order_account.payment_amount;
    let discount = settlement_fee_discount(SettlementFeeDiscountMode::ConfirmByPayer);

    settle_token_on_accept(
        &ctx.accounts.order_account.id.clone(),
        payment_amount,
        &ctx.accounts.order_account,
        &token_vault_info,
        &payer_token_info,
        &payee_token_info,
        &vault_token_info,
        &mint_info,
        &token_program_info,
        &payer_info,
        &payee_info,
        &payer_info,
        &ctx.accounts.payment_creator.to_account_info(),
        &fee_vault_info,
        &mut ctx.accounts.listing,
        discount.payer_bps_discount,
        discount.payee_bps_discount,
    )?;

    let clock = Clock::get()?;
    ctx.accounts.order_account.closed = true;
    ctx.accounts.order_account.closed_date = clock.unix_timestamp;
    ctx.accounts.order_account.closed_reason = OrderClosedReason::Confirmed;
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)] // id order is matter!!!
pub struct PayerConfirmTokenOrder<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub payee: SystemAccount<'info>,

    #[account(mut)]
    pub payment_creator: SystemAccount<'info>,

    #[account(
        mut,
        close = payment_creator,
        has_one = order_token_vault_account,
        has_one = mint_account,
        has_one = payer,
        has_one = payee,
        has_one = payer_token_account,
        has_one = payee_token_account,
        seeds = [
            SEED_ORDER_ACCOUNT,
            order_account.listing_id.as_ref(),
            order_account.instance_index.to_le_bytes().as_ref(),
        ],
        bump = order_account.bump,
    )]
    pub order_account: Box<Account<'info, Order>>,

    #[account(mut)]
    pub order_token_vault_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: Account key is validated against the payment and only used for CPI transfers.
    pub payer_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Account key is validated against the payment and only used for CPI transfers.
    pub payee_token_account: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = fee_vault_account,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,

    /// CHECK: Account is validated against config in instruction logic.
    pub fee_vault_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_LISTING_ACCOUNT, order_account.listing_id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        seeds = [SEED_LISTING_TOKEN_VAULT_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump_listing_token_vault,
    )]
    pub listing_token_vault_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
