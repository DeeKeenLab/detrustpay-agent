use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::{SEED_LISTING_ACCOUNT, SEED_LISTING_TOKEN_VAULT_ACCOUNT};
use crate::error::CustomError;
use crate::state::Listing;
use crate::ListingCapacityAdjusted;

use super::super::utils::creator_slot_amount;

const MAX_ACCEPT_CAPACITY: u64 = 9999;

pub fn process_adjust_listing_capacity_token(
    ctx: Context<AdjustListingCapacityToken>,
    capacity_delta: i64,
) -> Result<()> {
    let setup = &mut ctx.accounts.listing;
    require!(
        setup.creator == ctx.accounts.creator.key(),
        CustomError::SetupOnlyCreator
    );
    require!(
        setup.mint_account == ctx.accounts.mint_account.key(),
        CustomError::SetupMintMismatch
    );

    let current_capacity = i128::from(setup.accept_capacity);
    let used_capacity = i128::from(setup.used_capacity);
    let delta = i128::from(capacity_delta);
    let new_capacity = current_capacity
        .checked_add(delta)
        .ok_or(CustomError::AmountOverflow)?;
    require!(
        new_capacity >= used_capacity,
        CustomError::SetupCapacityExceeded
    );
    require!(new_capacity >= 0, CustomError::InvalidAcceptCapacity);
    require!(
        new_capacity <= i128::from(MAX_ACCEPT_CAPACITY),
        CustomError::InvalidAcceptCapacity
    );
    let new_accept_capacity =
        u64::try_from(new_capacity).map_err(|_| CustomError::InvalidAcceptCapacity)?;
    let (payer_key, payee_key) = if setup.is_payer_listing {
        (setup.creator, setup.counterparty)
    } else {
        (setup.counterparty, setup.creator)
    };

    let per_slot_creator = creator_slot_amount(
        setup.is_payer_listing,
        setup.payment_amount,
        setup.payer_deposit_amount,
        setup.payee_deposit_amount,
    )?;
    let remaining_capacity = new_accept_capacity
        .checked_sub(setup.used_capacity)
        .ok_or(CustomError::AmountUnderflow)?;
    let required_amount = per_slot_creator
        .checked_mul(remaining_capacity)
        .ok_or(CustomError::AmountOverflow)?;
    let vault_balance = ctx.accounts.listing_token_vault_account.amount;

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_LISTING_ACCOUNT, setup.id.as_ref(), &[setup.bump]]];

    let mut deposit_amount = 0;
    let mut refund_amount = 0;
    if capacity_delta < 0 {
        refund_amount = vault_balance
            .checked_sub(required_amount)
            .ok_or(CustomError::SetupInsufficientFunds)?;
        if refund_amount > 0 {
            let setup_info = setup.to_account_info();
            transfer_checked(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        from: ctx.accounts.listing_token_vault_account.to_account_info(),
                        mint: ctx.accounts.mint_account.to_account_info(),
                        to: ctx.accounts.creator_token_account.to_account_info(),
                        authority: setup_info,
                    },
                    signer_seeds,
                ),
                refund_amount,
                ctx.accounts.mint_account.decimals,
            )?;
        }

        setup.accept_capacity = new_accept_capacity;

        emit!(ListingCapacityAdjusted {
            setup: setup.key(),
            creator: ctx.accounts.creator.key(),
            payer: payer_key,
            payee: payee_key,
            mint: ctx.accounts.mint_account.key(),
            mint_decimals: ctx.accounts.mint_account.decimals,
            capacity_delta,
            new_accept_capacity: setup.accept_capacity,
            deposit_amount,
            refund_amount
        });
        return Ok(());
    }

    if vault_balance < required_amount {
        deposit_amount = required_amount
            .checked_sub(vault_balance)
            .ok_or(CustomError::AmountUnderflow)?;
        if deposit_amount > 0 {
            transfer_checked(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        from: ctx.accounts.creator_token_account.to_account_info(),
                        mint: ctx.accounts.mint_account.to_account_info(),
                        to: ctx.accounts.listing_token_vault_account.to_account_info(),
                        authority: ctx.accounts.creator.to_account_info(),
                    },
                ),
                deposit_amount,
                ctx.accounts.mint_account.decimals,
            )?;
        }
    }

    setup.accept_capacity = new_accept_capacity;

    emit!(ListingCapacityAdjusted {
        setup: setup.key(),
        creator: ctx.accounts.creator.key(),
        payer: payer_key,
        payee: payee_key,
        mint: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        capacity_delta,
        new_accept_capacity: setup.accept_capacity,
        deposit_amount,
        refund_amount,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct AdjustListingCapacityToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_LISTING_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = creator,
        associated_token::token_program = token_program,
    )]
    pub creator_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_LISTING_TOKEN_VAULT_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump_listing_token_vault,
    )]
    pub listing_token_vault_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
