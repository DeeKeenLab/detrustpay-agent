use anchor_lang::prelude::*;

use crate::error::CustomError;
use crate::state::Config;

pub fn validate_setup_flags(
    config: &Config,
    is_adjustable_payment: bool,
    is_custom_deposit: bool,
    payment_amount: u64,
    payer_deposit_amount: u64,
    payee_deposit_amount: u64,
) -> Result<()> {
    require!(
        payer_deposit_amount == payee_deposit_amount,
        CustomError::DepositMustMatch
    );
    if is_adjustable_payment {
        require!(
            config.enable_adjustable_payment,
            CustomError::AdjustablePaymentDisabled
        );
    }
    if is_custom_deposit {
        require!(
            config.enable_custom_deposit,
            CustomError::CustomDepositDisabled
        );
        validate_custom_deposit(payment_amount, payer_deposit_amount, payee_deposit_amount)?;
    } else {
        require!(
            payer_deposit_amount == payment_amount && payee_deposit_amount == payment_amount,
            CustomError::DepositMustMatchPayment
        );
    }
    Ok(())
}

pub fn validate_custom_deposit(
    payment_amount: u64,
    payer_deposit_amount: u64,
    payee_deposit_amount: u64,
) -> Result<()> {
    let max_deposit = payment_amount
        .checked_mul(4)
        .ok_or(CustomError::AmountOverflow)?;
    let payer_scaled = payer_deposit_amount
        .checked_mul(4)
        .ok_or(CustomError::AmountOverflow)?;
    let payee_scaled = payee_deposit_amount
        .checked_mul(4)
        .ok_or(CustomError::AmountOverflow)?;
    require!(
        payer_scaled >= payment_amount && payer_deposit_amount <= max_deposit,
        CustomError::CustomDepositOutOfRange
    );
    require!(
        payee_scaled >= payment_amount && payee_deposit_amount <= max_deposit,
        CustomError::CustomDepositOutOfRange
    );
    Ok(())
}

fn uuid_bytes_to_hex(id: &[u8; 16]) -> String {
    let mut s = String::with_capacity(32);
    for b in id {
        use core::fmt::Write;
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn validate_uuid_bytes(id: &[u8; 16]) -> Result<()> {
    let version = id[6] >> 4;
    let variant = id[8] >> 6;
    require!(variant == 0b10, CustomError::InvalidUuid);
    require!((1..=8).contains(&version), CustomError::InvalidUuid);
    Ok(())
}

pub fn build_instance_id(listing_id: &[u8; 16], instance_index: u64) -> Result<String> {
    let setup_uuid = uuid_bytes_to_hex(listing_id);
    Ok(format!("{}#{}", setup_uuid, instance_index))
}

pub fn creator_slot_amount(
    is_payer_listing: bool,
    payment_amount: u64,
    payer_deposit_amount: u64,
    payee_deposit_amount: u64,
) -> Result<u64> {
    if is_payer_listing {
        payment_amount
            .checked_add(payer_deposit_amount)
            .ok_or(CustomError::AmountOverflow.into())
    } else {
        Ok(payee_deposit_amount)
    }
}

pub fn counterparty_slot_amount(
    is_payer_listing: bool,
    payment_amount: u64,
    payer_deposit_amount: u64,
    payee_deposit_amount: u64,
) -> Result<u64> {
    if is_payer_listing {
        Ok(payee_deposit_amount)
    } else {
        payment_amount
            .checked_add(payer_deposit_amount)
            .ok_or(CustomError::AmountOverflow.into())
    }
}
