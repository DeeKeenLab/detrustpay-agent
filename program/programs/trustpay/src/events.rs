use anchor_lang::prelude::*;

#[event]
pub struct DirectTokenPaid {
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub amount: u64,
    pub fee: u64,
    pub from_account: Pubkey,
    pub to_account: Pubkey,
}

#[event]
pub struct OrderCreated {
    pub creator: Pubkey,
    pub id: String,
    pub buyer_order: Pubkey,
    pub seller_order: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub payment_amount: u64,
    pub payee_deposit_amount: u64,

    pub category: u8,
    pub details_url: String,
}
#[event]
pub struct OrderClosed {
    pub creator: Pubkey,
    pub id: String,
    pub payer: Pubkey,
    pub payee: Pubkey,
}

#[event]
pub struct OrderCancelled {
    pub creator: Pubkey,
    pub id: String,
    pub payment: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub fee: u64,
    pub fee_payer: u64,
    pub fee_payee: u64,
    pub refund_to_payer: u64,
    pub refund_to_payee: u64,
    pub drained_to_payer: u64,
    pub drained_to_payee: u64,
}

#[event]
pub struct OrderConfirmed {
    pub creator: Pubkey,
    pub id: String,
    pub payment: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub payment_amount: u64,
    pub fee: u64,
    pub fee_payer: u64,
    pub fee_payee: u64,
    pub to_payer: u64,
    pub to_payee: u64,
}

#[event]
pub struct OrderProposal {
    pub creator: Pubkey,
    pub id: String,
    pub payment: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub counter_amount: u64,
    pub version: u64,
}

#[event]
pub struct OrderProposalResponded {
    pub creator: Pubkey,
    pub id: String,
    pub payment: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub responder: Pubkey,
    pub is_accept: bool,
    pub counter_amount: u64,
    pub version: u64,
}

#[event]
pub struct PayerOrderMessageSet {
    pub creator: Pubkey,
    pub id: String,
    pub payment: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub message: String,
    pub is_encrypted: bool,
    pub payer_ephemeral_pubkey: Pubkey,
    pub message_nonce: [u8; 12],
}

#[event]
pub struct PayeeOrderMessageSet {
    pub creator: Pubkey,
    pub id: String,
    pub payment: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub message: String,
    pub is_encrypted: bool,
    pub payee_ephemeral_pubkey: Pubkey,
    pub message_nonce: [u8; 12],
}

#[event]
pub struct ListingCreated {
    pub setup: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub is_payer_listing: bool,
    pub counterparty: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub setup_token_vault: Pubkey,
    pub payment_amount: u64,
    pub payer_deposit_amount: u64,
    pub payee_deposit_amount: u64,
    pub accept_capacity: u64,
}

#[event]
pub struct ListingClosed {
    pub setup: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub remaining_amount: u64,
}

#[event]
pub struct ListingDeactivated {
    pub listing: Pubkey,
    pub creator: Pubkey,
    pub revision: u64,
}

#[event]
pub struct ListingVaultClosed {
    pub listing: Pubkey,
    pub creator: Pubkey,
    pub vault: Pubkey,
    pub remaining_amount: u64,
    pub revision: u64,
}

#[event]
pub struct ListingCapacityAdjusted {
    pub setup: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub capacity_delta: i64,
    pub new_accept_capacity: u64,
    pub deposit_amount: u64,
    pub refund_amount: u64,
}

#[event]
pub struct ConfigInitialized {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub manage_authority: Pubkey,
    pub enable_adjustable_payment: bool,
    pub enable_custom_deposit: bool,
    pub enable_dispute_deterrent: bool,
    pub paused: bool,
    pub slot: u64,
}

#[event]
pub struct ProtocolFeesWithdrawn {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub protocol_fee_vault: Pubkey,
    pub destination_token_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ConfigUpdated {
    pub field: String,
    pub previous_value: String,
    pub new_value: String,
}

#[event]
pub struct ProgramPauseUpdated {
    pub authority: Pubkey,
    pub previous_paused: bool,
    pub paused: bool,
    pub slot: u64,
}
