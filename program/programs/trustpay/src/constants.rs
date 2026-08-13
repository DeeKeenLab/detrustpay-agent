use anchor_lang::constant;

#[constant]
pub const SEED_ORDER_ACCOUNT: &[u8] = b"order";
#[constant]
pub const SEED_ORDER_TOKEN_VAULT_ACCOUNT: &[u8] = b"order_token_vault";
#[constant]
pub const SEED_LISTING_ACCOUNT: &[u8] = b"listing";
#[constant]
pub const SEED_LISTING_TOKEN_VAULT_ACCOUNT: &[u8] = b"listing_token_vault";
#[constant]
pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
pub const SECONDS_PER_WEEK: i64 = 7 * 24 * 60 * 60;
pub const MAX_MESSAGE_LENGTH: usize = 128;
