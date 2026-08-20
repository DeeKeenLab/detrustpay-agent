use anchor_lang::prelude::*;

use crate::constants::{SEED_BUYER_ORDER_COPY, SEED_PARTY_ORDER_ACCOUNT, SEED_SELLER_ORDER_COPY};
use crate::error::CustomError;
use crate::handlers::set_message::{apply_message_update, MessageRole};
use crate::state::{sync_order_pair, validate_order_pair, PartyOrder};
use crate::PayerOrderMessageSet;

pub fn process_set_payer_order_message(
    ctx: Context<SetPayerOrderMessage>,
    id: String,
    message: String,
    is_encrypted: bool,
    nonce: [u8; 12],
) -> Result<()> {
    validate_order_pair(
        &ctx.accounts.buyer_order_account,
        &ctx.accounts.seller_order_account,
    )?;
    require_keys_eq!(
        ctx.accounts.buyer_order_account.authority,
        ctx.accounts.payer.key(),
        CustomError::OrderAuthorityMismatch
    );

    let clock = Clock::get()?;
    apply_message_update(
        &mut ctx.accounts.buyer_order_account.order,
        &id,
        message,
        is_encrypted,
        nonce,
        MessageRole::Payer,
        clock.unix_timestamp,
    )?;
    ctx.accounts.buyer_order_account.order.version = ctx
        .accounts
        .buyer_order_account
        .order
        .version
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    sync_order_pair(
        &mut ctx.accounts.buyer_order_account,
        &mut ctx.accounts.seller_order_account,
    )?;

    emit!(PayerOrderMessageSet {
        creator: ctx.accounts.payer.key(),
        id: ctx.accounts.buyer_order_account.id.clone(),
        payment: ctx.accounts.buyer_order_account.key(),
        payer: ctx.accounts.payer.key(),
        payee: ctx.accounts.buyer_order_account.payee,
        message: ctx.accounts.buyer_order_account.payer_message.clone(),
        is_encrypted,
        payer_ephemeral_pubkey: ctx.accounts.buyer_order_account.payer_ephemeral_pubkey,
        message_nonce: ctx.accounts.buyer_order_account.payer_message_nonce,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct SetPayerOrderMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

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
}
