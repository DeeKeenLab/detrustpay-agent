use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum OrderClosedReason {
    Cancelled,
    Confirmed,
    PayerAcceptProposal,
    PayeeAcceptProposal,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Order {
    #[max_len(32)]
    pub id: String,
    pub listing_id: [u8; 16],
    #[max_len(64)]
    pub title: String,
    #[max_len(256)]
    pub description: String,

    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub parent_listing: Pubkey,
    pub instance_index: u64,
    pub is_adjustable_payment: bool,
    pub is_custom_deposit: bool,
    pub payment_amount: u64,
    pub payer_deposit_amount: u64,
    pub payee_deposit_amount: u64,
    pub payer_token_account: Pubkey,
    pub payee_token_account: Pubkey,
    pub order_token_vault_account: Pubkey,
    pub bump_order_token_vault_account: u8,
    pub mint_account: Pubkey,
    pub mint_decimals: u8,
    pub bump: u8,
    pub date_created: i64,
    pub date_accepted: i64,
    pub closed: bool,
    pub closed_date: i64,
    pub closed_reason: OrderClosedReason,
    pub version: u64,
    pub additional_fee_payer_bps: u64,
    pub additional_fee_payee_bps: u64,

    #[max_len(128)]
    pub payer_message: String,
    #[max_len(128)]
    pub payee_message: String,
    pub payer_message_is_encrypted: bool,
    pub payee_message_is_encrypted: bool,
    pub payer_message_nonce: [u8; 12],
    pub payee_message_nonce: [u8; 12],
    pub payer_ephemeral_pubkey: Pubkey,
    pub payee_ephemeral_pubkey: Pubkey,
    pub payer_message_date: i64,
    pub payee_message_date: i64,

    pub payer_made_proposal_amount: u64,
    pub payer_made_proposal_date: i64,
    pub payer_made_proposal_expiry: Option<i64>,

    pub payee_made_proposal_date: i64,
    pub payee_made_proposal_amount: u64,
    pub payee_made_proposal_expiry: Option<i64>,

    pub category: u8,
    #[max_len(128)]
    pub details_url: String,
}
