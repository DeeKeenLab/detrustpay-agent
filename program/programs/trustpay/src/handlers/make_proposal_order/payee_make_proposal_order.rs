use crate::constants::{MAX_MESSAGE_LENGTH, SEED_CONFIG_ACCOUNT, SEED_ORDER_ACCOUNT};
use crate::error::CustomError;
use crate::handlers::helpers::{add_proposal_penalty_bps, resolve_proposal_penalty_bps};
use crate::{
    state::{Config, Order},
    OrderProposal,
};
use anchor_lang::prelude::*;

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
    require!(!ctx.accounts.order_account.closed, CustomError::ClosedError);

    require!(
        ctx.accounts.order_account.id == id,
        CustomError::PaymentIdMismatch
    );
    require!(
        counter_amount <= ctx.accounts.order_account.payment_amount,
        CustomError::CounterAmountTooHigh
    );
    let clock = Clock::get()?;
    if let Some(expiry) = proposal_expiry {
        require!(
            expiry > clock.unix_timestamp,
            CustomError::ProposalExpiryInvalid
        );
    }
    let proposal_penalty_bps =
        resolve_proposal_penalty_bps(ctx.accounts.config_account.enable_dispute_deterrent)?;

    ctx.accounts.order_account.payee_made_proposal_amount = counter_amount;
    ctx.accounts.order_account.payee_made_proposal_expiry = proposal_expiry;
    ctx.accounts.order_account.version += 1;
    ctx.accounts.order_account.additional_fee_payee_bps = add_proposal_penalty_bps(
        ctx.accounts.order_account.additional_fee_payee_bps,
        proposal_penalty_bps,
    );
    ctx.accounts.order_account.additional_fee_payer_bps = add_proposal_penalty_bps(
        ctx.accounts.order_account.additional_fee_payer_bps,
        proposal_penalty_bps,
    );
    ctx.accounts.order_account.payee_made_proposal_date = clock.unix_timestamp;

    if let Some(message_value) = message {
        require!(
            message_value.len() <= MAX_MESSAGE_LENGTH,
            CustomError::MessageTooLong
        );
        ctx.accounts.order_account.payee_message = message_value;
        ctx.accounts.order_account.payee_message_is_encrypted = is_encrypted.unwrap_or(false);
        ctx.accounts.order_account.payee_message_nonce = nonce.unwrap_or([0u8; 12]);
        ctx.accounts.order_account.payee_message_date = clock.unix_timestamp;
        let message_ephemeral = ephemeral_pubkey.unwrap_or_default();
        if message_ephemeral != Pubkey::default() {
            ctx.accounts.order_account.payee_ephemeral_pubkey = message_ephemeral;
        }
    }

    emit!(OrderProposal {
        creator: ctx.accounts.payee.key(),
        id: ctx.accounts.order_account.id.clone(),
        payment: ctx.accounts.order_account.key(),
        payer: ctx.accounts.order_account.payer,
        payee: ctx.accounts.order_account.payee,
        counter_amount,
        version: ctx.accounts.order_account.version,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct PayeeMakeProposalOrder<'info> {
    #[account(mut)]
    pub payee: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        has_one = payee,
        seeds = [
            SEED_ORDER_ACCOUNT,
            order_account.listing_id.as_ref(),
            order_account.instance_index.to_le_bytes().as_ref(),
        ],
        bump = order_account.bump,
    )]
    pub order_account: Account<'info, Order>,
}
