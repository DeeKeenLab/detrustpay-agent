use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::constants::{SEED_CONFIG_ACCOUNT, SEED_LISTING_ACCOUNT, SEED_ORDER_ACCOUNT};
use crate::error::CustomError;
use crate::state::{require_fee_vault_account, Config, Listing, Order, OrderClosedReason};
use crate::{calc_payee_cancel_fee, OrderCancelled, PayeeCancelFee};

pub fn process_payee_cancel_token_order(
    ctx: Context<PayeeCancelTokenOrder>,
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

    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;
    let accepted_at = ctx.accounts.order_account.date_accepted;
    let PayeeCancelFee {
        fee_amount: raw_fee,
        ..
    } = calc_payee_cancel_fee(
        ctx.accounts.order_account.payment_amount,
        accepted_at,
        timestamp,
    )?;

    require!(
        ctx.accounts.order_account.parent_listing != Pubkey::default(),
        CustomError::MissingSetupAccount
    );

    let instance_index_bytes = ctx.accounts.order_account.instance_index.to_le_bytes();
    let payment_seeds = [
        SEED_ORDER_ACCOUNT,
        ctx.accounts.order_account.listing_id.as_ref(),
        instance_index_bytes.as_ref(),
        &[ctx.accounts.order_account.bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[&payment_seeds];

    let payer_refund = ctx
        .accounts
        .order_account
        .payer_deposit_amount
        .checked_add(ctx.accounts.order_account.payment_amount)
        .ok_or(CustomError::AmountOverflow)?;
    let fee_payee = raw_fee.min(ctx.accounts.order_account.payee_deposit_amount);
    let payee_refund = ctx
        .accounts
        .order_account
        .payee_deposit_amount
        .checked_sub(fee_payee)
        .ok_or(CustomError::AmountUnderflow)?;

    let setup = &ctx.accounts.listing;
    require_keys_eq!(
        setup.key(),
        ctx.accounts.order_account.parent_listing,
        CustomError::SetupAccountMismatch
    );
    require_keys_eq!(
        setup.listing_token_vault_account,
        ctx.accounts.listing_token_vault_account.key(),
        CustomError::SetupTokenVaultMismatch
    );
    require!(
        setup.mint_account == ctx.accounts.mint_account.key(),
        CustomError::SetupMintMismatch
    );
    require_keys_eq!(
        setup.creator,
        ctx.accounts.setup_creator.key(),
        CustomError::SetupAccountMismatch
    );

    let payer_token_info = ctx.accounts.payer_token_account.to_account_info();
    let payee_token_info = ctx.accounts.payee_token_account.to_account_info();
    let payer_destination = payer_token_info.clone();
    let payee_destination = payee_token_info.clone();

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.order_token_vault_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                to: payer_destination,
                authority: ctx.accounts.order_account.to_account_info(),
            },
            signer_seeds,
        ),
        payer_refund,
        ctx.accounts.mint_account.decimals,
    )?;

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.order_token_vault_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                to: payee_destination,
                authority: ctx.accounts.order_account.to_account_info(),
            },
            signer_seeds,
        ),
        payee_refund,
        ctx.accounts.mint_account.decimals,
    )?;

    if fee_payee > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.order_token_vault_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.order_account.to_account_info(),
                },
                signer_seeds,
            ),
            fee_payee,
            ctx.accounts.mint_account.decimals,
        )?;
    }

    let close_destination = ctx.accounts.payment_creator.to_account_info();

    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.order_token_vault_account.to_account_info(),
            destination: close_destination,
            authority: ctx.accounts.order_account.to_account_info(),
        },
        signer_seeds,
    ))?;

    // ctx.accounts.listing.used_capacity = ctx
    //     .accounts
    //     .listing
    //     .used_capacity
    //     .checked_sub(1)
    //     .ok_or(CustomError::AmountUnderflow)?;

    ctx.accounts.listing.active_orders = ctx
        .accounts
        .listing
        .active_orders
        .checked_sub(1)
        .ok_or(CustomError::AmountUnderflow)?;

    ctx.accounts.order_account.closed = true;
    ctx.accounts.order_account.closed_date = timestamp;
    ctx.accounts.order_account.closed_reason = OrderClosedReason::Cancelled;

    emit!(OrderCancelled {
        creator: ctx.accounts.payee.key(),
        id: ctx.accounts.order_account.id.clone(),
        payment: ctx.accounts.order_account.key(),
        payer: ctx.accounts.payer.key(),
        payee: ctx.accounts.payee.key(),
        vault: ctx.accounts.order_token_vault_account.key(),
        mint: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        fee: fee_payee,
        fee_payer: 0,
        fee_payee,
        refund_to_payer: payer_refund,
        refund_to_payee: payee_refund,
        drained_to_payer: 0,
        drained_to_payee: 0
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)] // id order is matter!!!
pub struct PayeeCancelTokenOrder<'info> {
    #[account(mut)]
    pub payee: Signer<'info>,

    #[account(mut)]
    pub payer: SystemAccount<'info>,

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
    /// CHECK: Account key is verified against the payment and only used as a CPI target.
    pub payer_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Account key is verified against the payment and only used as a CPI target.
    pub payee_token_account: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payee,
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

    #[account(mut)]
    pub listing_token_vault_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub setup_creator: SystemAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
