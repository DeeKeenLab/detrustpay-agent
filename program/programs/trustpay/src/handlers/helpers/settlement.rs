use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TransferChecked};

use crate::constants::SEED_ORDER_AUTHORITY;
use crate::error::CustomError;
use crate::state::{Listing, Order};
use crate::{calc_confirmation_fee, calc_fee_amount, ConfirmationFee, OrderConfirmed};

pub fn settle_token_on_accept<'info>(
    id: &str,
    payment_amount: u64,
    buyer_order_key: Pubkey,
    order_account: &Order,
    order_authority: &AccountInfo<'info>,
    order_token_vault_account: &AccountInfo<'info>,
    payer_token_account: &AccountInfo<'info>,
    payee_token_account: &AccountInfo<'info>,
    protocol_fee_vault: &AccountInfo<'info>,
    mint_account: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    payee: &AccountInfo<'info>,
    invoker: &AccountInfo<'info>,
    listing: &mut Account<'info, Listing>,
    payer_fee_bps_discount: u64,
    payee_fee_bps_discount: u64,
) -> Result<()> {
    require!(
        order_account.parent_listing != Pubkey::default(),
        CustomError::MissingSetupAccount
    );
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    let mut mint_data = &mint_account.data.borrow()[..];
    let mint = Mint::try_deserialize(&mut mint_data)?;

    let instance_index_bytes = order_account.instance_index.to_le_bytes();
    let payment_seeds = [
        SEED_ORDER_AUTHORITY,
        order_account.parent_listing.as_ref(),
        instance_index_bytes.as_ref(),
        &[order_account.bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[&payment_seeds];

    let accepted_at = order_account.date_accepted;
    let ConfirmationFee {
        total_fee: _computed_fee,
        payer_share: base_payer_fee_share,
        payee_share: base_payee_fee_share,
    } = calc_confirmation_fee(order_account.payment_amount, accepted_at, timestamp)?;
    let adjusted_payer_bps = order_account
        .additional_fee_payer_bps
        .saturating_sub(payer_fee_bps_discount);
    let adjusted_payee_bps = order_account
        .additional_fee_payee_bps
        .saturating_sub(payee_fee_bps_discount);
    let payer_extra_fee = calc_fee_amount(order_account.payment_amount, adjusted_payer_bps)?;
    let payee_extra_fee = calc_fee_amount(order_account.payment_amount, adjusted_payee_bps)?;
    let payer_fee_share = base_payer_fee_share
        .checked_add(payer_extra_fee)
        .ok_or(CustomError::AmountOverflow)?;
    let payee_fee_share = base_payee_fee_share
        .checked_add(payee_extra_fee)
        .ok_or(CustomError::AmountOverflow)?;
    let payer_available = order_account.payer_deposit_amount;
    let payee_available = order_account
        .payee_deposit_amount
        .checked_add(payment_amount)
        .ok_or(CustomError::AmountOverflow)?;
    let fee_payer = payer_fee_share.min(payer_available);
    let fee_payee = payee_fee_share.min(payee_available);
    let fee = fee_payer
        .checked_add(fee_payee)
        .ok_or(CustomError::AmountOverflow)?;
    let to_payer_amount = payer_available
        .checked_sub(fee_payer)
        .ok_or(CustomError::AmountUnderflow)?;
    let to_payee_amount = payee_available
        .checked_sub(fee_payee)
        .ok_or(CustomError::AmountUnderflow)?;
    let leftover = order_account
        .payment_amount
        .checked_sub(payment_amount)
        .ok_or(CustomError::AmountUnderflow)?;

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.clone(),
            TransferChecked {
                from: order_token_vault_account.clone(),
                mint: mint_account.clone(),
                to: payer_token_account.clone(),
                authority: order_authority.clone(),
            },
            signer_seeds,
        ),
        to_payer_amount,
        mint.decimals,
    )?;

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.clone(),
            TransferChecked {
                from: order_token_vault_account.clone(),
                mint: mint_account.clone(),
                to: payee_token_account.clone(),
                authority: order_authority.clone(),
            },
            signer_seeds,
        ),
        to_payee_amount,
        mint.decimals,
    )?;

    if fee > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                token_program.clone(),
                TransferChecked {
                    from: order_token_vault_account.clone(),
                    mint: mint_account.clone(),
                    to: protocol_fee_vault.clone(),
                    authority: order_authority.clone(),
                },
                signer_seeds,
            ),
            fee,
            mint.decimals,
        )?;
    }

    if leftover > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                token_program.clone(),
                TransferChecked {
                    from: order_token_vault_account.clone(),
                    mint: mint_account.clone(),
                    to: payer_token_account.clone(),
                    authority: order_authority.clone(),
                },
                signer_seeds,
            ),
            leftover,
            mint.decimals,
        )?;
    }

    let to_payer_total = to_payer_amount
        .checked_add(leftover)
        .ok_or(CustomError::AmountOverflow)?;

    require_keys_eq!(
        listing.key(),
        order_account.parent_listing,
        CustomError::SetupAccountMismatch
    );
    listing.active_orders = listing
        .active_orders
        .checked_sub(1)
        .ok_or(CustomError::AmountUnderflow)?;
    listing.revision = listing
        .revision
        .checked_add(1)
        .ok_or(CustomError::AmountOverflow)?;

    emit!(OrderConfirmed {
        creator: invoker.key(),
        id: id.to_string(),
        payment: buyer_order_key,
        payer: payer.key(),
        payee: payee.key(),
        vault: order_token_vault_account.key(),
        mint: mint_account.key(),
        mint_decimals: mint.decimals,
        payment_amount,
        fee,
        fee_payer,
        fee_payee,
        to_payer: to_payer_total,
        to_payee: to_payee_amount
    });

    Ok(())
}
