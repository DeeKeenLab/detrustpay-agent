use anchor_lang::prelude::*;

use crate::constants::{
    MAX_MESSAGE_LENGTH, SEED_BUYER_ORDER_COPY, SEED_PARTY_ORDER_ACCOUNT, SEED_SELLER_ORDER_COPY,
};
use crate::error::CustomError;
use crate::handlers::helpers::{add_proposal_penalty_bps, resolve_order_proposal_penalty_bps};
use crate::state::{sync_order_pair, validate_order_pair, PartyOrder};
use crate::OrderProposal;

pub fn process_payee_make_proposal_order(
    ctx: Context<PayeeMakeProposalOrder>,
    id: String,
    counter_amount: u64,
    proposal_expiry: Option<i64>,
    message: Option<String>,
    is_encrypted: Option<bool>,
    ephemeral_pubkey: Option<Pubkey>,
    nonce: Option<[u8; 12]>,
) -> Result<()> {
    validate_order_pair(
        &ctx.accounts.buyer_order_account,
        &ctx.accounts.seller_order_account,
    )?;
    require_keys_eq!(
        ctx.accounts.seller_order_account.authority,
        ctx.accounts.payee.key(),
        CustomError::OrderAuthorityMismatch
    );
    require!(
        !ctx.accounts.buyer_order_account.closed,
        CustomError::ClosedError
    );
    require!(
        ctx.accounts.buyer_order_account.id == id,
        CustomError::PaymentIdMismatch
    );
    require!(
        counter_amount <= ctx.accounts.buyer_order_account.payment_amount,
        CustomError::CounterAmountTooHigh
    );
    let clock = Clock::get()?;
    if let Some(expiry) = proposal_expiry {
        require!(
            expiry > clock.unix_timestamp,
            CustomError::ProposalExpiryInvalid
        );
    }
    let proposal_penalty_bps = resolve_order_proposal_penalty_bps(
        ctx.accounts.buyer_order_account.is_adjustable_payment,
        ctx.accounts.buyer_order_account.dispute_deterrent_enabled,
    )?;

    let order = &mut ctx.accounts.buyer_order_account.order;
    order.payee_made_proposal_amount = counter_amount;
    order.payee_made_proposal_expiry = proposal_expiry;
    order.version = order
        .version
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    order.additional_fee_payee_bps =
        add_proposal_penalty_bps(order.additional_fee_payee_bps, proposal_penalty_bps);
    order.additional_fee_payer_bps =
        add_proposal_penalty_bps(order.additional_fee_payer_bps, proposal_penalty_bps);
    order.payee_made_proposal_date = clock.unix_timestamp;

    if let Some(message_value) = message {
        require!(
            message_value.len() <= MAX_MESSAGE_LENGTH,
            CustomError::MessageTooLong
        );
        order.payee_message = message_value;
        order.payee_message_is_encrypted = is_encrypted.unwrap_or(false);
        order.payee_message_nonce = nonce.unwrap_or([0u8; 12]);
        order.payee_message_date = clock.unix_timestamp;
        let message_ephemeral = ephemeral_pubkey.unwrap_or_default();
        if message_ephemeral != Pubkey::default() {
            order.payee_ephemeral_pubkey = message_ephemeral;
        }
    }

    sync_order_pair(
        &mut ctx.accounts.buyer_order_account,
        &mut ctx.accounts.seller_order_account,
    )?;
    emit!(OrderProposal {
        creator: ctx.accounts.payee.key(),
        id: ctx.accounts.buyer_order_account.id.clone(),
        payment: ctx.accounts.buyer_order_account.key(),
        payer: ctx.accounts.buyer_order_account.payer,
        payee: ctx.accounts.buyer_order_account.payee,
        counter_amount,
        version: ctx.accounts.buyer_order_account.version,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct PayeeMakeProposalOrder<'info> {
    #[account(mut)]
    pub payee: Signer<'info>,

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
