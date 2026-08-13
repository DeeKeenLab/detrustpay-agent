use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::{
    SEED_CONFIG_ACCOUNT, SEED_LISTING_ACCOUNT, SEED_LISTING_TOKEN_VAULT_ACCOUNT,
};
use crate::error::CustomError;
use crate::state::{require_program_active, Config, Listing};
use crate::ListingCreated;

use super::super::utils::{creator_slot_amount, validate_setup_flags, validate_uuid_bytes};

pub fn process_create_listing_token(
    ctx: Context<CreateListingToken>,
    id: [u8; 16],
    title: String,
    description: String,
    ephemeral_pubkey: Pubkey,
    is_payer_listing: bool,
    payment_amount: u64,
    payer_deposit_amount: u64,
    payee_deposit_amount: u64,
    accept_capacity: u64,
    expiration: Option<i64>,
    is_adjustable_payment: bool,
    is_custom_deposit: bool,
    counterparty: Option<Pubkey>,
) -> Result<()> {
    require_program_active(&ctx.accounts.config_account)?;
    validate_uuid_bytes(&id)?;
    require!(
        ephemeral_pubkey != Pubkey::default(),
        CustomError::MissingEncryptionPubkey
    );
    let config = &ctx.accounts.config_account;
    let max_accept_capacity: u64 = 9999;
    require!(accept_capacity > 0, CustomError::InvalidAcceptCapacity);
    require!(
        accept_capacity <= max_accept_capacity,
        CustomError::InvalidAcceptCapacity
    );
    validate_setup_flags(
        config,
        is_adjustable_payment,
        is_custom_deposit,
        payment_amount,
        payer_deposit_amount,
        payee_deposit_amount,
    )?;

    let counterparty_key = counterparty.unwrap_or_default();
    if counterparty_key != Pubkey::default() {
        require!(
            counterparty_key != ctx.accounts.creator.key(),
            CustomError::SetupCounterpartyMismatch
        );
    }

    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    let expiration = if let Some(expiration) = expiration {
        if expiration <= 0 {
            return err!(CustomError::InvalidSetupExpiration);
        }
        require!(expiration >= timestamp, CustomError::InvalidSetupExpiration);
        expiration
    } else {
        0
    };

    let per_slot_amount = creator_slot_amount(
        is_payer_listing,
        payment_amount,
        payer_deposit_amount,
        payee_deposit_amount,
    )?;
    let total_amount = per_slot_amount
        .checked_mul(accept_capacity)
        .ok_or(CustomError::AmountOverflow)?;
    let (payer_key, payee_key) = if is_payer_listing {
        (ctx.accounts.creator.key(), counterparty_key)
    } else {
        (counterparty_key, ctx.accounts.creator.key())
    };

    *ctx.accounts.listing = Listing {
        id,
        title,
        description,
        creator: ctx.accounts.creator.key(),
        creator_ephemeral_pubkey: ephemeral_pubkey,
        is_payer_listing,
        counterparty: counterparty_key,
        mint_account: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        creator_token_account: ctx.accounts.creator_token_account.key(),
        payment_amount,
        payer_deposit_amount,
        payee_deposit_amount,
        accept_capacity,
        used_capacity: 0,
        active_orders: 0,
        next_order_index: 1,
        is_adjustable_payment,
        is_custom_deposit,
        listing_token_vault_account: ctx.accounts.listing_token_vault_account.key(),
        bump: ctx.bumps.listing,
        bump_listing_token_vault: ctx.bumps.listing_token_vault_account,
        date_created: timestamp,
        expiration,
    };

    if total_amount > 0 {
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
            total_amount,
            ctx.accounts.mint_account.decimals,
        )?;
    }

    emit!(ListingCreated {
        setup: ctx.accounts.listing.key(),
        creator: ctx.accounts.creator.key(),
        payer: payer_key,
        payee: payee_key,
        is_payer_listing,
        counterparty: counterparty_key,
        mint: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        setup_token_vault: ctx.accounts.listing_token_vault_account.key(),
        payment_amount,
        payer_deposit_amount,
        payee_deposit_amount,
        accept_capacity
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: [u8; 16])]
pub struct CreateListingToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Listing::INIT_SPACE + Listing::DISCRIMINATOR.len(),
        seeds = [SEED_LISTING_ACCOUNT, id.as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = creator,
        associated_token::token_program = token_program,
    )]
    pub creator_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = creator,
        seeds = [SEED_LISTING_TOKEN_VAULT_ACCOUNT, id.as_ref()],
        bump,
        token::mint = mint_account,
        token::authority = listing,
        token::token_program = token_program,
    )]
    pub listing_token_vault_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
