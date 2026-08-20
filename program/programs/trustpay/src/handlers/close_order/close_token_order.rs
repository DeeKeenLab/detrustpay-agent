use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, CloseAccount, TokenAccount, TokenInterface};

use crate::constants::{
    SEED_BUYER_ORDER_COPY, SEED_ORDER_AUTHORITY, SEED_ORDER_TOKEN_VAULT_ACCOUNT,
    SEED_PARTY_ORDER_ACCOUNT, SEED_SELLER_ORDER_COPY,
};
use crate::error::CustomError;
use crate::state::{sync_order_pair, validate_order_pair, PartyOrder, PartyRole};
use crate::OrderClosed;

pub fn process_close_order_vault(ctx: Context<CloseOrderVault>) -> Result<()> {
    validate_order_pair(
        &ctx.accounts.buyer_order_account,
        &ctx.accounts.seller_order_account,
    )?;
    let order = &ctx.accounts.buyer_order_account.order;
    require!(order.closed, CustomError::PaymentNotClosed);
    require!(!order.vault_closed, CustomError::OrderVaultAlreadyClosed);
    require!(
        ctx.accounts.order_token_vault_account.amount == 0,
        CustomError::OrderVaultNotEmpty
    );
    require_keys_eq!(
        order.order_token_vault_account,
        ctx.accounts.order_token_vault_account.key(),
        CustomError::TokenVaultMismatch
    );
    require_keys_eq!(
        order.vault_rent_refund_recipient,
        ctx.accounts.vault_rent_refund_recipient.key(),
        CustomError::RecipientMismatch
    );
    require!(
        ctx.accounts.participant.key() == order.payer
            || ctx.accounts.participant.key() == order.payee,
        CustomError::OrderParticipantOnly
    );

    let instance_index_bytes = order.instance_index.to_le_bytes();
    let authority_seeds = [
        SEED_ORDER_AUTHORITY,
        order.parent_listing.as_ref(),
        instance_index_bytes.as_ref(),
        &[order.bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[&authority_seeds];
    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.order_token_vault_account.to_account_info(),
            destination: ctx.accounts.vault_rent_refund_recipient.to_account_info(),
            authority: ctx.accounts.order_authority.to_account_info(),
        },
        signer_seeds,
    ))?;

    let order = &mut ctx.accounts.buyer_order_account.order;
    order.vault_closed = true;
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

pub fn process_close_my_order_copy(ctx: Context<CloseMyOrderCopy>) -> Result<()> {
    let order_copy = &ctx.accounts.order_copy;
    require_keys_eq!(
        order_copy.authority,
        ctx.accounts.authority.key(),
        CustomError::OrderAuthorityMismatch
    );
    require!(order_copy.closed, CustomError::PaymentNotClosed);
    require!(order_copy.vault_closed, CustomError::OrderVaultNotClosed);

    let role_seed = match order_copy.role {
        PartyRole::Buyer => SEED_BUYER_ORDER_COPY,
        PartyRole::Seller => SEED_SELLER_ORDER_COPY,
    };
    let (expected, _) = Pubkey::find_program_address(
        &[
            SEED_PARTY_ORDER_ACCOUNT,
            order_copy.parent_listing.as_ref(),
            order_copy.instance_index.to_le_bytes().as_ref(),
            role_seed,
        ],
        &crate::ID,
    );
    require_keys_eq!(expected, order_copy.key(), CustomError::OrderPairMismatch);

    emit!(OrderClosed {
        creator: ctx.accounts.authority.key(),
        id: order_copy.id.clone(),
        payer: order_copy.payer,
        payee: order_copy.payee,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CloseOrderVault<'info> {
    pub participant: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_PARTY_ORDER_ACCOUNT, buyer_order_account.parent_listing.as_ref(), buyer_order_account.instance_index.to_le_bytes().as_ref(), SEED_BUYER_ORDER_COPY],
        bump = buyer_order_account.bump,
    )]
    pub buyer_order_account: Box<Account<'info, PartyOrder>>,

    #[account(
        mut,
        seeds = [SEED_PARTY_ORDER_ACCOUNT, seller_order_account.parent_listing.as_ref(), seller_order_account.instance_index.to_le_bytes().as_ref(), SEED_SELLER_ORDER_COPY],
        bump = seller_order_account.bump,
    )]
    pub seller_order_account: Box<Account<'info, PartyOrder>>,

    /// CHECK: Accountless PDA constrained by shared order identity.
    #[account(
        seeds = [SEED_ORDER_AUTHORITY, buyer_order_account.parent_listing.as_ref(), buyer_order_account.instance_index.to_le_bytes().as_ref()],
        bump = buyer_order_account.order.bump,
    )]
    pub order_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_ORDER_TOKEN_VAULT_ACCOUNT, buyer_order_account.parent_listing.as_ref(), buyer_order_account.instance_index.to_le_bytes().as_ref()],
        bump = buyer_order_account.bump_order_token_vault_account,
    )]
    pub order_token_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault_rent_refund_recipient: SystemAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct CloseMyOrderCopy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, close = authority, has_one = authority)]
    pub order_copy: Account<'info, PartyOrder>,
}
