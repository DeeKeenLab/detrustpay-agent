use anchor_lang::prelude::*;

use crate::constants::SEED_ORDER_ACCOUNT;
use crate::handlers::set_message::{apply_message_update, MessageRole};
use crate::{state::Order, PayeeOrderMessageSet};

pub fn process_set_payee_order_message(
    ctx: Context<SetPayeeOrderMessage>,
    id: String,
    message: String,
    is_encrypted: bool,
    nonce: [u8; 12],
) -> Result<()> {
    let clock = Clock::get()?;
    apply_message_update(
        &mut ctx.accounts.order_account,
        &id,
        message,
        is_encrypted,
        nonce,
        MessageRole::Payee,
        clock.unix_timestamp,
    )?;
    emit!(PayeeOrderMessageSet {
        creator: ctx.accounts.payee.key(),
        id: ctx.accounts.order_account.id.clone(),
        payment: ctx.accounts.order_account.key(),
        payer: ctx.accounts.order_account.payer,
        payee: ctx.accounts.payee.key(),
        message: ctx.accounts.order_account.payee_message.clone(),
        is_encrypted,
        payee_ephemeral_pubkey: ctx.accounts.order_account.payee_ephemeral_pubkey,
        message_nonce: ctx.accounts.order_account.payee_message_nonce,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct SetPayeeOrderMessage<'info> {
    #[account(mut)]
    pub payee: Signer<'info>,

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
