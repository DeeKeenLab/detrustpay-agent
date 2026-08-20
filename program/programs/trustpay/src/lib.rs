#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod handlers;
pub mod state;

pub use events::*;
pub use handlers::*;

declare_id!("3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL");

#[program]
pub mod detrustpay {
    use super::*;
    pub fn create_listing_token(
        ctx: Context<CreateListingToken>,
        id: [u8; 16],
        title: String,
        description: String,
        category: u8,
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
        process_create_listing_token(
            ctx,
            id,
            title,
            description,
            category,
            ephemeral_pubkey,
            is_payer_listing,
            payment_amount,
            payer_deposit_amount,
            payee_deposit_amount,
            accept_capacity,
            expiration,
            is_adjustable_payment,
            is_custom_deposit,
            counterparty,
        )
    }

    pub fn accept_listing_token(
        ctx: Context<AcceptListingToken>,
        listing_id: [u8; 16],
        message: Option<String>,
        is_encrypted: Option<bool>,
        ephemeral_pubkey: Pubkey,
        nonce: Option<[u8; 12]>,
        details_url: String,
    ) -> Result<()> {
        process_accept_listing_token(
            ctx,
            listing_id,
            message,
            is_encrypted,
            ephemeral_pubkey,
            nonce,
            details_url,
        )
    }

    pub fn deactivate_listing(ctx: Context<DeactivateListing>) -> Result<()> {
        process_deactivate_listing(ctx)
    }

    pub fn close_listing_vault(ctx: Context<CloseListingVault>) -> Result<()> {
        process_close_listing_vault(ctx)
    }

    pub fn close_listing(ctx: Context<CloseListing>) -> Result<()> {
        process_close_listing(ctx)
    }

    pub fn adjust_listing_capacity_token(
        ctx: Context<AdjustListingCapacityToken>,
        capacity_delta: i64,
    ) -> Result<()> {
        process_adjust_listing_capacity_token(ctx, capacity_delta)
    }

    pub fn payee_cancel_token_order(ctx: Context<PayeeCancelTokenOrder>, id: String) -> Result<()> {
        process_payee_cancel_token_order(ctx, id)
    }

    pub fn payer_confirm_token_order(
        ctx: Context<PayerConfirmTokenOrder>,
        id: String,
    ) -> Result<()> {
        process_payer_confirm_token_order(ctx, id)
    }

    pub fn close_order_vault(ctx: Context<CloseOrderVault>) -> Result<()> {
        process_close_order_vault(ctx)
    }

    pub fn close_my_order_copy(ctx: Context<CloseMyOrderCopy>) -> Result<()> {
        process_close_my_order_copy(ctx)
    }

    pub fn payer_make_proposal_order(
        ctx: Context<PayerMakeProposalOrder>,
        id: String,
        counter_amount: u64,
        proposal_expiry: Option<i64>,
        message: Option<String>,
        is_encrypted: Option<bool>,
        ephemeral_pubkey: Option<Pubkey>,
        nonce: Option<[u8; 12]>,
    ) -> Result<()> {
        process_payer_make_proposal_order(
            ctx,
            id,
            counter_amount,
            proposal_expiry,
            message,
            is_encrypted,
            ephemeral_pubkey,
            nonce,
        )
    }

    pub fn payee_make_proposal_order(
        ctx: Context<PayeeMakeProposalOrder>,
        id: String,
        counter_amount: u64,
        proposal_expiry: Option<i64>,
        message: Option<String>,
        is_encrypted: Option<bool>,
        ephemeral_pubkey: Option<Pubkey>,
        nonce: Option<[u8; 12]>,
    ) -> Result<()> {
        process_payee_make_proposal_order(
            ctx,
            id,
            counter_amount,
            proposal_expiry,
            message,
            is_encrypted,
            ephemeral_pubkey,
            nonce,
        )
    }

    pub fn payer_accept_proposal_token_order(
        ctx: Context<PayerAcceptProposalTokenOrder>,
        id: String,
        version: u64,
    ) -> Result<()> {
        process_payer_accept_proposal_token_order(ctx, id, version)
    }

    pub fn payee_accept_proposal_token_order(
        ctx: Context<PayeeAcceptProposalTokenOrder>,
        id: String,
        version: u64,
    ) -> Result<()> {
        process_payee_accept_proposal_token_order(ctx, id, version)
    }

    pub fn set_payer_order_message(
        ctx: Context<SetPayerOrderMessage>,
        id: String,
        message: String,
        is_encrypted: bool,
        nonce: [u8; 12],
    ) -> Result<()> {
        process_set_payer_order_message(ctx, id, message, is_encrypted, nonce)
    }

    pub fn set_payee_order_message(
        ctx: Context<SetPayeeOrderMessage>,
        id: String,
        message: String,
        is_encrypted: bool,
        nonce: [u8; 12],
    ) -> Result<()> {
        process_set_payee_order_message(ctx, id, message, is_encrypted, nonce)
    }

    pub fn direct_pay_token(
        ctx: Context<DirectPayToken>,
        amount: u64,
        payee: Pubkey,
    ) -> Result<()> {
        process_direct_pay_token(ctx, amount, payee)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        manage_authority: Pubkey,
        enable_adjustable_payment: bool,
        enable_custom_deposit: bool,
        enable_dispute_deterrent: bool,
    ) -> Result<()> {
        process_initialize_config(
            ctx,
            manage_authority,
            enable_adjustable_payment,
            enable_custom_deposit,
            enable_dispute_deterrent,
        )
    }

    pub fn update_enable_adjustable_payment(
        ctx: Context<UpdateConfigByManageAuthority>,
        enabled: bool,
    ) -> Result<()> {
        process_update_enable_adjustable_payment(ctx, enabled)
    }

    pub fn update_enable_custom_deposit(
        ctx: Context<UpdateConfigByManageAuthority>,
        enabled: bool,
    ) -> Result<()> {
        process_update_enable_custom_deposit(ctx, enabled)
    }

    pub fn update_enable_dispute_deterrent(
        ctx: Context<UpdateConfigByManageAuthority>,
        enabled: bool,
    ) -> Result<()> {
        process_update_enable_dispute_deterrent(ctx, enabled)
    }

    pub fn update_manage_authority(
        ctx: Context<UpdateConfigByManageAuthority>,
        new_manage_authority: Pubkey,
    ) -> Result<()> {
        process_update_manage_authority(ctx, new_manage_authority)
    }

    pub fn update_program_paused(
        ctx: Context<UpdateConfigByManageAuthority>,
        paused: bool,
    ) -> Result<()> {
        process_update_program_paused(ctx, paused)
    }

    pub fn withdraw_protocol_fees(ctx: Context<WithdrawProtocolFees>, amount: u64) -> Result<()> {
        process_withdraw_protocol_fees(ctx, amount)
    }
}
