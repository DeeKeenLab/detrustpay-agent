use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::{
    MAX_MESSAGE_LENGTH, SEED_BUYER_ORDER_COPY, SEED_CONFIG_ACCOUNT, SEED_LISTING_ACCOUNT,
    SEED_LISTING_TOKEN_VAULT_ACCOUNT, SEED_ORDER_AUTHORITY, SEED_ORDER_TOKEN_VAULT_ACCOUNT,
    SEED_PARTY_ORDER_ACCOUNT, SEED_SELLER_ORDER_COPY,
};
use crate::error::CustomError;
use crate::state::{
    require_program_active, Config, Listing, Order, OrderClosedReason, PartyOrder, PartyRole,
};
use crate::OrderCreated;

use super::super::utils::{
    build_instance_id, counterparty_slot_amount, creator_slot_amount, validate_uuid_bytes,
};

const MAX_DETAILS_URL_LENGTH: usize = 128;

pub fn process_accept_listing_token(
    ctx: Context<AcceptListingToken>,
    listing_id: [u8; 16],
    message: Option<String>,
    is_encrypted: Option<bool>,
    ephemeral_pubkey: Pubkey,
    nonce: Option<[u8; 12]>,
    details_url: String,
) -> Result<()> {
    require_program_active(&ctx.accounts.config_account)?;
    validate_uuid_bytes(&listing_id)?;
    require!(
        details_url.len() <= MAX_DETAILS_URL_LENGTH,
        CustomError::DetailsUrlTooLong
    );

    let setup = &mut ctx.accounts.listing;
    require!(setup.is_active, CustomError::SetupExpired);
    require!(
        setup.used_capacity < setup.accept_capacity,
        CustomError::SetupCapacityExceeded
    );
    if setup.counterparty != Pubkey::default() {
        require_keys_eq!(
            setup.counterparty,
            ctx.accounts.counterparty.key(),
            CustomError::SetupCounterpartyMismatch
        );
    }
    require_keys_neq!(
        setup.creator,
        ctx.accounts.counterparty.key(),
        CustomError::SetupCounterpartyMismatch
    );
    require_keys_eq!(
        setup.mint_account,
        ctx.accounts.mint_account.key(),
        CustomError::SetupMintMismatch
    );
    require_keys_eq!(
        setup.listing_token_vault_account,
        ctx.accounts.listing_token_vault_account.key(),
        CustomError::SetupTokenVaultMismatch
    );
    require_keys_eq!(
        ctx.accounts.listing_token_vault_account.mint,
        ctx.accounts.mint_account.key(),
        CustomError::SetupMintMismatch
    );
    require!(
        setup.creator_ephemeral_pubkey != Pubkey::default(),
        CustomError::MissingEncryptionPubkey
    );
    require!(
        ephemeral_pubkey != Pubkey::default(),
        CustomError::MissingEncryptionPubkey
    );

    let per_slot_creator = creator_slot_amount(
        setup.is_payer_listing,
        setup.payment_amount,
        setup.payer_deposit_amount,
        setup.payee_deposit_amount,
    )?;
    require!(
        ctx.accounts.listing_token_vault_account.amount >= per_slot_creator,
        CustomError::SetupInsufficientFunds
    );

    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;
    if setup.expiration > 0 {
        require!(timestamp <= setup.expiration, CustomError::SetupExpired);
    }
    let instance_index = setup.next_order_index;
    let instance_id = build_instance_id(&setup.id, instance_index)?;

    let (payer_key, payee_key, payer_token_key, payee_token_key) = if setup.is_payer_listing {
        (
            setup.creator,
            ctx.accounts.counterparty.key(),
            setup.creator_token_account,
            ctx.accounts.counterparty_token_account.key(),
        )
    } else {
        (
            ctx.accounts.counterparty.key(),
            setup.creator,
            ctx.accounts.counterparty_token_account.key(),
            setup.creator_token_account,
        )
    };

    let mut payer_message = String::new();
    let mut payee_message = String::new();
    let mut payer_message_time = 0;
    let mut payee_message_time = 0;
    let mut payer_message_is_encrypted = false;
    let mut payee_message_is_encrypted = false;
    let mut payer_message_nonce = [0u8; 12];
    let mut payee_message_nonce = [0u8; 12];
    let payer_ephemeral_pubkey = if setup.is_payer_listing {
        setup.creator_ephemeral_pubkey
    } else {
        ephemeral_pubkey
    };
    let payee_ephemeral_pubkey = if setup.is_payer_listing {
        ephemeral_pubkey
    } else {
        setup.creator_ephemeral_pubkey
    };

    if let Some(message) = message {
        require!(
            message.len() <= MAX_MESSAGE_LENGTH,
            CustomError::MessageTooLong
        );
        let message_is_encrypted = is_encrypted.unwrap_or(false);
        let message_nonce = nonce.unwrap_or([0u8; 12]);

        if setup.is_payer_listing {
            payee_message = message;
            payee_message_is_encrypted = message_is_encrypted;
            payee_message_nonce = message_nonce;
            payee_message_time = timestamp;
        } else {
            payer_message = message;
            payer_message_is_encrypted = message_is_encrypted;
            payer_message_nonce = message_nonce;
            payer_message_time = timestamp;
        }
    }

    let order = Box::new(Order {
        id: instance_id.clone(),
        listing_id: setup.id,
        title: setup.title.clone(),
        description: setup.description.clone(),
        payer_message,
        payee_message,
        payer_message_is_encrypted,
        payee_message_is_encrypted,
        payer_message_nonce,
        payee_message_nonce,
        payer_ephemeral_pubkey,
        payee_ephemeral_pubkey,
        creator: ctx.accounts.counterparty.key(),
        payer: payer_key,
        payee: payee_key,
        parent_listing: setup.key(),
        instance_index,
        is_adjustable_payment: setup.is_adjustable_payment,
        is_custom_deposit: setup.is_custom_deposit,
        dispute_deterrent_enabled: setup.dispute_deterrent_enabled,
        payment_amount: setup.payment_amount,
        payer_deposit_amount: setup.payer_deposit_amount,
        payee_deposit_amount: setup.payee_deposit_amount,
        payer_token_account: payer_token_key,
        payee_token_account: payee_token_key,
        order_token_vault_account: ctx.accounts.order_token_vault_account.key(),
        bump_order_token_vault_account: ctx.bumps.order_token_vault_account,
        bump: ctx.bumps.order_authority,
        vault_rent_refund_recipient: ctx.accounts.counterparty.key(),
        vault_closed: false,
        mint_account: ctx.accounts.mint_account.key(),
        mint_decimals: setup.mint_decimals,
        payer_made_proposal_amount: setup.payment_amount,
        payee_made_proposal_amount: setup.payment_amount,
        date_created: timestamp,
        date_accepted: timestamp,
        closed: false,
        closed_date: 0,
        closed_reason: OrderClosedReason::Cancelled,
        payer_made_proposal_date: 0,
        payee_made_proposal_date: 0,
        payer_message_date: payer_message_time,
        payee_message_date: payee_message_time,
        version: 1,
        additional_fee_payer_bps: 0,
        additional_fee_payee_bps: 0,
        payer_made_proposal_expiry: None,
        payee_made_proposal_expiry: None,
        category: setup.category,
        details_url: details_url.clone(),
    });

    let buyer_order = &mut ctx.accounts.buyer_order_account;
    buyer_order.authority = payer_key;
    buyer_order.counterparty = payee_key;
    buyer_order.role = PartyRole::Buyer;
    buyer_order.bump = ctx.bumps.buyer_order_account;
    buyer_order.order = (*order).clone();
    buyer_order.refresh_digest()?;

    let seller_order = &mut ctx.accounts.seller_order_account;
    seller_order.authority = payee_key;
    seller_order.counterparty = payer_key;
    seller_order.role = PartyRole::Seller;
    seller_order.bump = ctx.bumps.seller_order_account;
    seller_order.order = *order;
    seller_order.refresh_digest()?;

    // The accepter fronts both inits because the listing creator is offline. The
    // same atomic instruction reimburses the creator-side copy from creator-funded
    // excess lamports held in the Listing account.
    let creator_copy_rent =
        Rent::get()?.minimum_balance(PartyOrder::INIT_SPACE + PartyOrder::DISCRIMINATOR.len());
    require!(
        setup.creator_order_rent_reserve >= creator_copy_rent,
        CustomError::ListingRentReserveInsufficient
    );
    setup.creator_order_rent_reserve = setup
        .creator_order_rent_reserve
        .checked_sub(creator_copy_rent)
        .ok_or(CustomError::AmountUnderflow)?;
    let counterparty_amount = counterparty_slot_amount(
        setup.is_payer_listing,
        setup.payment_amount,
        setup.payer_deposit_amount,
        setup.payee_deposit_amount,
    )?;

    let setup_signer_seeds: &[&[&[u8]]] =
        &[&[SEED_LISTING_ACCOUNT, listing_id.as_ref(), &[setup.bump]]];
    if per_slot_creator > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.listing_token_vault_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.order_token_vault_account.to_account_info(),
                    authority: setup.to_account_info(),
                },
                setup_signer_seeds,
            ),
            per_slot_creator,
            ctx.accounts.mint_account.decimals,
        )?;
    }
    if counterparty_amount > 0 {
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.counterparty_token_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.order_token_vault_account.to_account_info(),
                    authority: ctx.accounts.counterparty.to_account_info(),
                },
            ),
            counterparty_amount,
            ctx.accounts.mint_account.decimals,
        )?;
    }

    // Direct lamport movement must happen after every CPI. Otherwise the
    // runtime observes an outstanding balance change at the next invocation.
    let setup_info = setup.to_account_info();
    let counterparty_info = ctx.accounts.counterparty.to_account_info();
    let setup_lamports = setup_info.lamports();
    let counterparty_lamports = counterparty_info.lamports();
    let new_setup_lamports = setup_lamports
        .checked_sub(creator_copy_rent)
        .ok_or(CustomError::AmountUnderflow)?;
    let new_counterparty_lamports = counterparty_lamports
        .checked_add(creator_copy_rent)
        .ok_or(CustomError::AmountOverflow)?;
    let mut setup_lamports_ref = setup_info.try_borrow_mut_lamports()?;
    let mut counterparty_lamports_ref = counterparty_info.try_borrow_mut_lamports()?;
    **setup_lamports_ref = new_setup_lamports;
    **counterparty_lamports_ref = new_counterparty_lamports;
    drop(counterparty_lamports_ref);
    drop(setup_lamports_ref);

    setup.used_capacity = setup
        .used_capacity
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    setup.active_orders = setup
        .active_orders
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    setup.next_order_index = setup
        .next_order_index
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;
    setup.revision = setup
        .revision
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;

    let listing_rent =
        Rent::get()?.minimum_balance(Listing::INIT_SPACE + Listing::DISCRIMINATOR.len());
    require!(
        setup.to_account_info().lamports()
            >= listing_rent
                .checked_add(setup.creator_order_rent_reserve)
                .ok_or(CustomError::AmountOverflow)?,
        CustomError::ListingRentReserveInsufficient
    );

    emit!(OrderCreated {
        creator: ctx.accounts.counterparty.key(),
        id: instance_id,
        buyer_order: ctx.accounts.buyer_order_account.key(),
        seller_order: ctx.accounts.seller_order_account.key(),
        payer: payer_key,
        payee: payee_key,
        vault: ctx.accounts.order_token_vault_account.key(),
        mint: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        payment_amount: setup.payment_amount,
        payee_deposit_amount: setup.payee_deposit_amount,
        category: setup.category,
        details_url,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(listing_id: [u8; 16])]
pub struct AcceptListingToken<'info> {
    #[account(mut)]
    pub counterparty: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [SEED_LISTING_ACCOUNT, listing_id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        init,
        payer = counterparty,
        space = PartyOrder::INIT_SPACE + PartyOrder::DISCRIMINATOR.len(),
        seeds = [
            SEED_PARTY_ORDER_ACCOUNT,
            listing.key().as_ref(),
            listing.next_order_index.to_le_bytes().as_ref(),
            SEED_BUYER_ORDER_COPY,
        ],
        bump,
    )]
    pub buyer_order_account: Box<Account<'info, PartyOrder>>,

    #[account(
        init,
        payer = counterparty,
        space = PartyOrder::INIT_SPACE + PartyOrder::DISCRIMINATOR.len(),
        seeds = [
            SEED_PARTY_ORDER_ACCOUNT,
            listing.key().as_ref(),
            listing.next_order_index.to_le_bytes().as_ref(),
            SEED_SELLER_ORDER_COPY,
        ],
        bump,
    )]
    pub seller_order_account: Box<Account<'info, PartyOrder>>,

    /// CHECK: Accountless PDA used only as the token-vault signing authority.
    #[account(
        seeds = [
            SEED_ORDER_AUTHORITY,
            listing.key().as_ref(),
            listing.next_order_index.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub order_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = counterparty,
        seeds = [
            SEED_ORDER_TOKEN_VAULT_ACCOUNT,
            listing.key().as_ref(),
            listing.next_order_index.to_le_bytes().as_ref()
        ],
        bump,
        token::mint = mint_account,
        token::authority = order_authority,
        token::token_program = token_program,
    )]
    pub order_token_vault_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = counterparty,
        associated_token::token_program = token_program,
    )]
    pub counterparty_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [SEED_LISTING_TOKEN_VAULT_ACCOUNT, listing_id.as_ref()],
        bump = listing.bump_listing_token_vault,
    )]
    pub listing_token_vault_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
