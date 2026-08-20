use anchor_lang::constant;

#[constant]
pub const SEED_PARTY_ORDER_ACCOUNT: &[u8] = b"party_order";
pub const SEED_BUYER_ORDER_COPY: &[u8] = b"buyer";
pub const SEED_SELLER_ORDER_COPY: &[u8] = b"seller";
pub const SEED_ORDER_AUTHORITY: &[u8] = b"order_authority";
#[constant]
pub const SEED_ORDER_TOKEN_VAULT_ACCOUNT: &[u8] = b"order_token_vault";
#[constant]
pub const SEED_LISTING_ACCOUNT: &[u8] = b"listing";
#[constant]
pub const SEED_LISTING_TOKEN_VAULT_ACCOUNT: &[u8] = b"listing_token_vault";
#[constant]
pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
#[constant]
pub const SEED_PROTOCOL_FEE_VAULT_ACCOUNT: &[u8] = b"protocol_fee_vault";
pub const SECONDS_PER_WEEK: i64 = 7 * 24 * 60 * 60;
pub const MAX_MESSAGE_LENGTH: usize = 128;
