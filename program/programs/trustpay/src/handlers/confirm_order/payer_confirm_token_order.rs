use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::{
    SEED_BUYER_ORDER_COPY, SEED_CONFIG_ACCOUNT, SEED_LISTING_ACCOUNT, SEED_ORDER_AUTHORITY,
    SEED_ORDER_TOKEN_VAULT_ACCOUNT, SEED_PARTY_ORDER_ACCOUNT, SEED_PROTOCOL_FEE_VAULT_ACCOUNT,
    SEED_SELLER_ORDER_COPY,
};
use crate::error::CustomError;
use crate::handlers::helpers::{
    settle_token_on_accept, settlement_fee_discount, SettlementFeeDiscountMode,
};
use crate::state::{
    sync_order_pair, validate_order_pair, Config, Listing, OrderClosedReason, PartyOrder,
};

pub fn process_payer_confirm_token_order(
    ctx: Context<PayerConfirmTokenOrder>,
    id: String,
) -> Result<()> {
    validate_order_pair(
        &ctx.accounts.buyer_order_account,
        &ctx.accounts.seller_order_account,
    )?;
    let order = &ctx.accounts.buyer_order_account.order;
    require_keys_eq!(
        order.payer,
        ctx.accounts.payer.key(),
        CustomError::OrderAuthorityMismatch
    );
    require_keys_eq!(
        order.payee,
        ctx.accounts.payee.key(),
        CustomError::OrderAuthorityMismatch
    );
    require!(!order.closed, CustomError::ClosedError);
    require!(order.id == id, CustomError::PaymentIdMismatch);
    require_keys_eq!(
        order.order_token_vault_account,
        ctx.accounts.order_token_vault_account.key(),
        CustomError::TokenVaultMismatch
    );
    require_keys_eq!(
        order.mint_account,
        ctx.accounts.mint_account.key(),
        CustomError::MintAccountMismatch
    );
    require_keys_eq!(
        order.payer_token_account,
        ctx.accounts.payer_token_account.key(),
        CustomError::TokenAccountMismatch
    );
    require_keys_eq!(
        order.payee_token_account,
        ctx.accounts.payee_token_account.key(),
        CustomError::TokenAccountMismatch
    );

    let payment_amount = order.payment_amount;
    let discount = settlement_fee_discount(SettlementFeeDiscountMode::ConfirmByPayer);
    settle_token_on_accept(
        &order.id.clone(),
        payment_amount,
        ctx.accounts.buyer_order_account.key(),
        order,
        &ctx.accounts.order_authority.to_account_info(),
        &ctx.accounts.order_token_vault_account.to_account_info(),
        &ctx.accounts.payer_token_account.to_account_info(),
        &ctx.accounts.payee_token_account.to_account_info(),
        &ctx.accounts.protocol_fee_vault.to_account_info(),
        &ctx.accounts.mint_account.to_account_info(),
        &ctx.accounts.token_program.to_account_info(),
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.payee.to_account_info(),
        &ctx.accounts.payer.to_account_info(),
        &mut ctx.accounts.listing,
        discount.payer_bps_discount,
        discount.payee_bps_discount,
    )?;

    let clock = Clock::get()?;
    let order = &mut ctx.accounts.buyer_order_account.order;
    order.closed = true;
    order.closed_date = clock.unix_timestamp;
    order.closed_reason = OrderClosedReason::Confirmed;
    order.version = order
        .version
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    sync_order_pair(
        &mut ctx.accounts.buyer_order_account,
        &mut ctx.accounts.seller_order_account,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct PayerConfirmTokenOrder<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub payee: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PARTY_ORDER_ACCOUNT,
            buyer_order_account.parent_listing.as_ref(),
            buyer_order_account.instance_index.to_le_bytes().as_ref(),
            SEED_BUYER_ORDER_COPY,
        ],
        bump = buyer_order_account.bump,
    )]
    pub buyer_order_account: Box<Account<'info, PartyOrder>>,

    #[account(
        mut,
        seeds = [
            SEED_PARTY_ORDER_ACCOUNT,
            seller_order_account.parent_listing.as_ref(),
            seller_order_account.instance_index.to_le_bytes().as_ref(),
            SEED_SELLER_ORDER_COPY,
        ],
        bump = seller_order_account.bump,
    )]
    pub seller_order_account: Box<Account<'info, PartyOrder>>,

    /// CHECK: Accountless PDA constrained by the shared order identity.
    #[account(
        seeds = [
            SEED_ORDER_AUTHORITY,
            buyer_order_account.parent_listing.as_ref(),
            buyer_order_account.instance_index.to_le_bytes().as_ref(),
        ],
        bump = buyer_order_account.order.bump,
    )]
    pub order_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_ORDER_TOKEN_VAULT_ACCOUNT,
            buyer_order_account.parent_listing.as_ref(),
            buyer_order_account.instance_index.to_le_bytes().as_ref(),
        ],
        bump = buyer_order_account.bump_order_token_vault_account,
    )]
    pub order_token_vault_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: Key is checked against shared order state.
    pub payer_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Key is checked against shared order state.
    pub payee_token_account: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [SEED_PROTOCOL_FEE_VAULT_ACCOUNT, mint_account.key().as_ref()],
        bump,
        token::mint = mint_account,
        token::authority = config_account,
        token::token_program = token_program,
    )]
    pub protocol_fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_LISTING_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
