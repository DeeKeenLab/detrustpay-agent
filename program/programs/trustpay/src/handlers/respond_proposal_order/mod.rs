use crate::error::CustomError;
use crate::state::Order;
use crate::OrderProposalResponded;
use anchor_lang::prelude::*;

pub mod payee_accept_proposal_token_order;
pub use payee_accept_proposal_token_order::*;
pub mod payer_accept_proposal_token_order;
pub use payer_accept_proposal_token_order::*;

fn ensure_payee_can_respond<'info>(order_account: &Account<'info, Order>, id: &str) -> Result<()> {
    require!(!order_account.closed, CustomError::ClosedError);

    require!(order_account.id == id, CustomError::PaymentIdMismatch);
    require!(
        order_account.payer_made_proposal_date > 0,
        CustomError::ProposalNotAvailable
    );
    if let Some(expiry) = order_account.payer_made_proposal_expiry {
        let clock = Clock::get()?;
        require!(expiry > clock.unix_timestamp, CustomError::ProposalExpired);
    }
    Ok(())
}

fn apply_payee_response<'info>(
    order_account: &mut Account<'info, Order>,
    id: &str,
    payment_key: Pubkey,
    responder: Pubkey,
    is_accept: bool,
    counter_amount: u64,
) -> Result<()> {
    if !is_accept {
        order_account.payer_made_proposal_amount = order_account.payment_amount;
    }
    order_account.version = order_account
        .version
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;

    emit!(OrderProposalResponded {
        creator: responder,
        id: id.to_string(),
        payment: payment_key,
        payer: order_account.payer,
        payee: order_account.payee,
        responder,
        is_accept,
        counter_amount,
        version: order_account.version,
    });

    Ok(())
}

fn ensure_payer_can_respond<'info>(order_account: &Account<'info, Order>, id: &str) -> Result<()> {
    require!(!order_account.closed, CustomError::ClosedError);

    require!(order_account.id == id, CustomError::PaymentIdMismatch);
    require!(
        order_account.payee_made_proposal_date > 0,
        CustomError::ProposalNotAvailable
    );
    if let Some(expiry) = order_account.payee_made_proposal_expiry {
        let clock = Clock::get()?;
        require!(expiry > clock.unix_timestamp, CustomError::ProposalExpired);
    }
    Ok(())
}

fn apply_payer_response<'info>(
    order_account: &mut Account<'info, Order>,
    id: &str,
    payment_key: Pubkey,
    responder: Pubkey,
    is_accept: bool,
    counter_amount: u64,
) -> Result<()> {
    if !is_accept {
        order_account.payee_made_proposal_amount = order_account.payment_amount;
    }
    order_account.version = order_account
        .version
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;

    emit!(OrderProposalResponded {
        creator: responder,
        id: id.to_string(),
        payment: payment_key,
        payer: order_account.payer,
        payee: order_account.payee,
        responder,
        is_accept,
        counter_amount,
        version: order_account.version,
    });

    Ok(())
}
