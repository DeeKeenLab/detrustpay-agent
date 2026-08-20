use std::ops::{Deref, DerefMut};

use anchor_lang::prelude::*;
use solana_sha256_hasher::hash;

use crate::error::CustomError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
#[cfg_attr(test, derive(Default))]
pub enum PartyRole {
    #[cfg_attr(test, default)]
    Buyer,
    Seller,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
#[cfg_attr(test, derive(Default))]
pub enum OrderClosedReason {
    #[cfg_attr(test, default)]
    Cancelled,
    Confirmed,
    PayerAcceptProposal,
    PayeeAcceptProposal,
}

/// Complete shared order state. Both PartyOrder accounts carry an identical copy.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
#[cfg_attr(test, derive(Default))]
pub struct Order {
    pub payer: Pubkey,
    pub payee: Pubkey,

    #[max_len(32)]
    pub id: String,
    pub listing_id: [u8; 16],
    #[max_len(64)]
    pub title: String,
    #[max_len(256)]
    pub description: String,

    pub creator: Pubkey,
    pub parent_listing: Pubkey,
    pub instance_index: u64,
    pub is_adjustable_payment: bool,
    pub is_custom_deposit: bool,
    pub dispute_deterrent_enabled: bool,
    pub payment_amount: u64,
    pub payer_deposit_amount: u64,
    pub payee_deposit_amount: u64,
    pub payer_token_account: Pubkey,
    pub payee_token_account: Pubkey,
    pub order_token_vault_account: Pubkey,
    pub bump_order_token_vault_account: u8,
    /// Bump for the accountless OrderAuthority PDA that signs token-vault CPIs.
    pub bump: u8,
    pub vault_rent_refund_recipient: Pubkey,
    pub vault_closed: bool,
    pub mint_account: Pubkey,
    pub mint_decimals: u8,
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

#[account]
#[derive(InitSpace, Debug)]
#[cfg_attr(test, derive(Default))]
pub struct PartyOrder {
    // Searchable fixed-width prefix. Authority stays at byte offset 8.
    pub authority: Pubkey,
    pub counterparty: Pubkey,
    pub role: PartyRole,
    pub state_digest: [u8; 32],
    pub bump: u8,
    pub order: Order,
}

impl PartyOrder {
    pub const AUTHORITY_MEMCMP_OFFSET: usize = 8;

    pub fn new(
        authority: Pubkey,
        counterparty: Pubkey,
        role: PartyRole,
        bump: u8,
        order: Order,
    ) -> Result<Self> {
        let state_digest = order_state_digest(&order)?;
        Ok(Self {
            authority,
            counterparty,
            role,
            state_digest,
            bump,
            order,
        })
    }

    pub fn refresh_digest(&mut self) -> Result<()> {
        self.state_digest = order_state_digest(&self.order)?;
        Ok(())
    }
}

impl Deref for PartyOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for PartyOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

pub fn order_state_digest(order: &Order) -> Result<[u8; 32]> {
    let serialized = order
        .try_to_vec()
        .map_err(|_| error!(CustomError::OrderStateSerializationFailed))?;
    Ok(hash(&serialized).to_bytes())
}

pub fn validate_order_pair(buyer: &PartyOrder, seller: &PartyOrder) -> Result<()> {
    require!(
        buyer.role == PartyRole::Buyer,
        CustomError::OrderRoleMismatch
    );
    require!(
        seller.role == PartyRole::Seller,
        CustomError::OrderRoleMismatch
    );
    require_keys_eq!(
        buyer.authority,
        buyer.payer,
        CustomError::OrderAuthorityMismatch
    );
    require_keys_eq!(
        seller.authority,
        seller.payee,
        CustomError::OrderAuthorityMismatch
    );
    require_keys_eq!(
        buyer.counterparty,
        seller.authority,
        CustomError::OrderPairMismatch
    );
    require_keys_eq!(
        seller.counterparty,
        buyer.authority,
        CustomError::OrderPairMismatch
    );
    require!(buyer.order == seller.order, CustomError::OrderPairMismatch);

    let expected_digest = order_state_digest(&buyer.order)?;
    require!(
        buyer.state_digest == expected_digest,
        CustomError::OrderDigestMismatch
    );
    require!(
        seller.state_digest == expected_digest,
        CustomError::OrderDigestMismatch
    );
    Ok(())
}

/// Copy the buyer-side transition result into the seller mirror and refresh both
/// digests. Call validate_order_pair before mutating the buyer copy.
pub fn sync_order_pair(buyer: &mut PartyOrder, seller: &mut PartyOrder) -> Result<()> {
    seller.order = buyer.order.clone();
    let digest = order_state_digest(&buyer.order)?;
    buyer.state_digest = digest;
    seller.state_digest = digest;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair() -> (PartyOrder, PartyOrder) {
        let payer = Pubkey::new_unique();
        let payee = Pubkey::new_unique();
        let order = Order {
            payer,
            payee,
            ..Order::default()
        };
        (
            PartyOrder::new(payer, payee, PartyRole::Buyer, 1, order.clone()).unwrap(),
            PartyOrder::new(payee, payer, PartyRole::Seller, 2, order).unwrap(),
        )
    }

    #[test]
    fn authority_has_stable_rpc_filter_offset() {
        let (buyer, _) = pair();
        let mut account_data = Vec::new();
        buyer.try_serialize(&mut account_data).unwrap();

        assert_eq!(
            &account_data
                [PartyOrder::AUTHORITY_MEMCMP_OFFSET..PartyOrder::AUTHORITY_MEMCMP_OFFSET + 32],
            buyer.authority.as_ref()
        );
    }

    #[test]
    fn valid_pair_is_accepted() {
        let (buyer, seller) = pair();
        validate_order_pair(&buyer, &seller).unwrap();
    }

    #[test]
    fn divergent_pair_is_rejected() {
        let (buyer, mut seller) = pair();
        seller.order.payment_amount = 7;
        assert!(validate_order_pair(&buyer, &seller).is_err());
    }

    #[test]
    fn sync_makes_shared_state_and_digest_identical() {
        let (mut buyer, mut seller) = pair();
        buyer.order.version = 9;
        sync_order_pair(&mut buyer, &mut seller).unwrap();
        validate_order_pair(&buyer, &seller).unwrap();
        assert_eq!(seller.order.version, 9);
    }
}
