use anchor_lang::prelude::*;

use crate::error::CustomError;
use crate::state::Order;

pub const MAX_MESSAGE_LENGTH: usize = 128;

pub enum MessageRole {
    Payer,
    Payee,
}

pub fn apply_message_update(
    payment: &mut Order,
    id: &str,
    message: String,
    is_encrypted: bool,
    nonce: [u8; 12],
    role: MessageRole,
    timestamp: i64,
) -> Result<()> {
    require!(!payment.closed, CustomError::ClosedError);
    require!(
        message.len() <= MAX_MESSAGE_LENGTH,
        CustomError::MessageTooLong
    );
    require!(payment.id == id, CustomError::PaymentIdMismatch);

    match role {
        MessageRole::Payer => {
            payment.payer_message = message;
            payment.payer_message_is_encrypted = is_encrypted;
            payment.payer_message_nonce = nonce;
            payment.payer_message_date = timestamp;
        }
        MessageRole::Payee => {
            payment.payee_message = message;
            payment.payee_message_is_encrypted = is_encrypted;
            payment.payee_message_nonce = nonce;
            payment.payee_message_date = timestamp;
        }
    }

    Ok(())
}

pub mod set_payer_order_message;
pub use set_payer_order_message::*;

pub mod set_payee_order_message;
pub use set_payee_order_message::*;
