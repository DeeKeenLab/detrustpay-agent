use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Listing {
    pub id: [u8; 16],
    #[max_len(64)]
    pub title: String,
    #[max_len(256)]
    pub description: String,
    pub creator: Pubkey,
    pub creator_ephemeral_pubkey: Pubkey,
    pub is_payer_listing: bool,
    pub counterparty: Pubkey,
    pub mint_account: Pubkey,
    pub mint_decimals: u8,
    pub creator_token_account: Pubkey,
    pub payment_amount: u64,
    pub payer_deposit_amount: u64,
    pub payee_deposit_amount: u64,
    pub accept_capacity: u64,
    pub used_capacity: u64,
    pub active_orders: u64,
    pub next_order_index: u64,
    pub is_adjustable_payment: bool,
    pub is_custom_deposit: bool,
    pub listing_token_vault_account: Pubkey,
    pub bump: u8,
    pub bump_listing_token_vault: u8,
    pub date_created: i64,
    pub expiration: i64,
}
