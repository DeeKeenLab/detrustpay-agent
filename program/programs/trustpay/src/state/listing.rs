use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Listing {
    // Searchable fixed-width prefix.
    pub creator: Pubkey,
    pub counterparty: Pubkey,
    pub mint_account: Pubkey,
    pub is_active: bool,
    pub is_payer_listing: bool,
    pub category: u8,
    pub accept_capacity: u64,
    pub used_capacity: u64,
    pub active_orders: u64,
    pub next_order_index: u64,
    pub revision: u64,

    pub id: [u8; 16],
    #[max_len(64)]
    pub title: String,
    #[max_len(256)]
    pub description: String,
    pub creator_ephemeral_pubkey: Pubkey,
    pub mint_decimals: u8,
    pub creator_token_account: Pubkey,
    pub payment_amount: u64,
    pub payer_deposit_amount: u64,
    pub payee_deposit_amount: u64,
    pub is_adjustable_payment: bool,
    pub is_custom_deposit: bool,
    pub dispute_deterrent_enabled: bool,
    pub listing_token_vault_account: Pubkey,
    pub creator_order_rent_reserve: u64,
    pub listing_vault_closed: bool,
    pub bump: u8,
    pub bump_listing_token_vault: u8,
    pub date_created: i64,
    pub expiration: i64,
}

impl Listing {
    pub const CREATOR_MEMCMP_OFFSET: usize = 8;
    pub const COUNTERPARTY_MEMCMP_OFFSET: usize = Self::CREATOR_MEMCMP_OFFSET + 32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creator_and_counterparty_have_stable_rpc_offsets() {
        let creator = Pubkey::new_unique();
        let counterparty = Pubkey::new_unique();
        let listing = Listing {
            creator,
            counterparty,
            mint_account: Pubkey::default(),
            is_active: true,
            is_payer_listing: true,
            category: 0,
            accept_capacity: 1,
            used_capacity: 0,
            active_orders: 0,
            next_order_index: 1,
            revision: 1,
            id: [0; 16],
            title: String::new(),
            description: String::new(),
            creator_ephemeral_pubkey: Pubkey::default(),
            mint_decimals: 6,
            creator_token_account: Pubkey::default(),
            payment_amount: 0,
            payer_deposit_amount: 0,
            payee_deposit_amount: 0,
            is_adjustable_payment: false,
            is_custom_deposit: false,
            dispute_deterrent_enabled: false,
            listing_token_vault_account: Pubkey::default(),
            creator_order_rent_reserve: 0,
            listing_vault_closed: false,
            bump: 0,
            bump_listing_token_vault: 0,
            date_created: 0,
            expiration: 0,
        };
        let mut account_data = Vec::new();
        listing.try_serialize(&mut account_data).unwrap();
        assert_eq!(
            &account_data[Listing::CREATOR_MEMCMP_OFFSET..Listing::CREATOR_MEMCMP_OFFSET + 32],
            creator.as_ref()
        );
        assert_eq!(
            &account_data
                [Listing::COUNTERPARTY_MEMCMP_OFFSET..Listing::COUNTERPARTY_MEMCMP_OFFSET + 32],
            counterparty.as_ref()
        );
    }
}
