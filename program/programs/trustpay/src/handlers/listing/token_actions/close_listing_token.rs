use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::constants::{SEED_LISTING_ACCOUNT, SEED_LISTING_TOKEN_VAULT_ACCOUNT};
use crate::error::CustomError;
use crate::state::Listing;
use crate::{ListingClosed, ListingDeactivated, ListingVaultClosed};

pub fn process_deactivate_listing(ctx: Context<DeactivateListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    require_keys_eq!(
        listing.creator,
        ctx.accounts.creator.key(),
        CustomError::SetupOnlyCreator
    );
    listing.is_active = false;
    listing.revision = listing
        .revision
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    emit!(ListingDeactivated {
        listing: listing.key(),
        creator: ctx.accounts.creator.key(),
        revision: listing.revision,
    });
    Ok(())
}

pub fn process_close_listing_vault(ctx: Context<CloseListingVault>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    require_keys_eq!(
        listing.creator,
        ctx.accounts.creator.key(),
        CustomError::SetupOnlyCreator
    );
    require!(listing.active_orders == 0, CustomError::SetupNotEmpty);
    require!(
        !listing.listing_vault_closed,
        CustomError::OrderVaultAlreadyClosed
    );
    require_keys_eq!(
        listing.mint_account,
        ctx.accounts.mint_account.key(),
        CustomError::SetupMintMismatch
    );
    require_keys_eq!(
        listing.listing_token_vault_account,
        ctx.accounts.listing_token_vault_account.key(),
        CustomError::SetupTokenVaultMismatch
    );

    listing.is_active = false;
    let remaining_amount = ctx.accounts.listing_token_vault_account.amount;
    let signer_seeds: &[&[&[u8]]] =
        &[&[SEED_LISTING_ACCOUNT, listing.id.as_ref(), &[listing.bump]]];

    if remaining_amount > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.listing_token_vault_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.creator_token_account.to_account_info(),
                    authority: listing.to_account_info(),
                },
                signer_seeds,
            ),
            remaining_amount,
            ctx.accounts.mint_account.decimals,
        )?;
    }

    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.listing_token_vault_account.to_account_info(),
            destination: ctx.accounts.creator.to_account_info(),
            authority: listing.to_account_info(),
        },
        signer_seeds,
    ))?;

    listing.listing_vault_closed = true;
    listing.revision = listing
        .revision
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    emit!(ListingVaultClosed {
        listing: listing.key(),
        creator: ctx.accounts.creator.key(),
        vault: ctx.accounts.listing_token_vault_account.key(),
        remaining_amount,
        revision: listing.revision,
    });
    Ok(())
}

pub fn process_close_listing(ctx: Context<CloseListing>) -> Result<()> {
    let listing = &ctx.accounts.listing;
    require_keys_eq!(
        listing.creator,
        ctx.accounts.creator.key(),
        CustomError::SetupOnlyCreator
    );
    require!(!listing.is_active, CustomError::PaymentCloseNotReady);
    require!(listing.active_orders == 0, CustomError::SetupNotEmpty);
    require!(
        listing.listing_vault_closed,
        CustomError::OrderVaultNotClosed
    );

    emit!(ListingClosed {
        setup: listing.key(),
        creator: ctx.accounts.creator.key(),
        payer: if listing.is_payer_listing {
            listing.creator
        } else {
            listing.counterparty
        },
        payee: if listing.is_payer_listing {
            listing.counterparty
        } else {
            listing.creator
        },
        mint: listing.mint_account,
        mint_decimals: listing.mint_decimals,
        remaining_amount: listing.creator_order_rent_reserve,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct DeactivateListing<'info> {
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_LISTING_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
}

#[derive(Accounts)]
pub struct CloseListingVault<'info> {
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

#[derive(Accounts)]
pub struct CloseListing<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        close = creator,
        seeds = [SEED_LISTING_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
}
