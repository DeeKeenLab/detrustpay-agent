use anchor_lang::prelude::*;

use crate::constants::SEED_ORDER_ACCOUNT;
use crate::error::CustomError;
use crate::state::Order;
use crate::OrderClosed;

pub fn process_close_token_order(ctx: Context<CloseTokenOrder>) -> Result<()> {
    let payment = &ctx.accounts.order_account;
    require!(payment.closed, CustomError::PaymentNotClosed);

    require_keys_eq!(
        payment.creator,
        ctx.accounts.creator.key(),
        CustomError::PaymentOnlyCreator
    );
    emit!(OrderClosed {
        creator: ctx.accounts.creator.key(),
        id: ctx.accounts.order_account.id.clone(),
        payer: ctx.accounts.order_account.key(),
        payee: ctx.accounts.order_account.key(),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CloseTokenOrder<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        close = creator,
        seeds = [
            SEED_ORDER_ACCOUNT,
            order_account.listing_id.as_ref(),
            order_account.instance_index.to_le_bytes().as_ref(),
        ],
        bump = order_account.bump,
    )]
    pub order_account: Account<'info, Order>,
}
