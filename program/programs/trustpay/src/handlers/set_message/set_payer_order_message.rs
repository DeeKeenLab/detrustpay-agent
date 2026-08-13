use anchor_lang::prelude::*;

use crate::constants::SEED_ORDER_ACCOUNT;
use crate::handlers::set_message::{apply_message_update, MessageRole};

use crate::{state::Order, PayerOrderMessageSet};

pub fn process_set_payer_order_message(
    ctx: Context<SetPayerOrderMessage>,
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
        MessageRole::Payer,
        clock.unix_timestamp,
    )?;
    emit!(PayerOrderMessageSet {
        creator: ctx.accounts.payer.key(),
        id: ctx.accounts.order_account.id.clone(),
        payment: ctx.accounts.order_account.key(),
        payer: ctx.accounts.payer.key(),
        payee: ctx.accounts.order_account.payee,
        message: ctx.accounts.order_account.payer_message.clone(),
        is_encrypted,
        payer_ephemeral_pubkey: ctx.accounts.order_account.payer_ephemeral_pubkey,
        message_nonce: ctx.accounts.order_account.payer_message_nonce,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct SetPayerOrderMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        has_one = payer,
        seeds = [
            SEED_ORDER_ACCOUNT,
            order_account.listing_id.as_ref(),
            order_account.instance_index.to_le_bytes().as_ref(),
        ],
        bump = order_account.bump,
    )]
    pub order_account: Account<'info, Order>,
}
