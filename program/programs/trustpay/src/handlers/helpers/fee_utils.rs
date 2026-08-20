use anchor_lang::prelude::*;

use crate::constants::SECONDS_PER_WEEK;
use crate::error::CustomError;

pub struct ConfirmationFee {
    pub total_fee: u64,
    pub payer_share: u64,
    pub payee_share: u64,
}

pub struct PayeeCancelFee {
    pub fee_amount: u64,
    pub fee_bps: u64,
}

pub struct SettlementFeeDiscount {
    pub payer_bps_discount: u64,
    pub payee_bps_discount: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettlementFeeDiscountMode {
    ConfirmByPayer,
    PayerAcceptPayeeProposal,
    PayeeAcceptPayerProposal,
}

const BASIS_POINTS_DENOMINATOR: u64 = 10_000;
const CONFIRMATION_BASE_BPS: u64 = 50; // 0.5%
const CONFIRMATION_FREE_WEEKS: u64 = 4;
const CONFIRMATION_WEEKLY_BPS_AFTER_FREE: u64 = 100; // 1% per full week after free period
const PAYEE_CANCEL_BASE_BPS: u64 = 500; // 5%
const PAYEE_CANCEL_WEEKLY_BPS: u64 = 100; // 1% per full week
pub const PROPOSAL_PENALTY_BPS: u64 = 20; // 0.2%
pub const MAX_PROPOSAL_PENALTY_BPS: u64 = 10_000; // 100%
pub const PROPOSAL_MAKER_FEE_BPS: u64 = PROPOSAL_PENALTY_BPS;
pub const PROPOSAL_COUNTERPARTY_FEE_BPS: u64 = PROPOSAL_PENALTY_BPS;
pub const PAYER_CONFIRM_FEE_REDUCTION_BPS: u64 = 20; // 0.2%

fn calc_elapsed_weeks(accepted_at: i64, now: i64) -> u64 {
    if now <= accepted_at {
        return 0;
    }
    let elapsed = now - accepted_at;
    (elapsed as u64) / SECONDS_PER_WEEK as u64
}

pub fn calc_fee_amount(payment_amount: u64, fee_bps: u64) -> Result<u64> {
    let amount = (payment_amount as u128)
        .checked_mul(fee_bps as u128)
        .ok_or(CustomError::AmountOverflow)?
        .checked_div(BASIS_POINTS_DENOMINATOR as u128)
        .ok_or(CustomError::AmountUnderflow)?;
    u64::try_from(amount).map_err(|_| CustomError::AmountOverflow.into())
}

pub fn resolve_proposal_penalty_bps(enable_dispute_deterrent: bool) -> Result<u64> {
    if !enable_dispute_deterrent {
        return Ok(0);
    }
    require!(
        PROPOSAL_PENALTY_BPS > 0 && PROPOSAL_PENALTY_BPS <= MAX_PROPOSAL_PENALTY_BPS,
        CustomError::InvalidProposalPenaltyConfig
    );
    Ok(PROPOSAL_PENALTY_BPS)
}

pub fn resolve_order_proposal_penalty_bps(
    is_adjustable_payment: bool,
    dispute_deterrent_enabled: bool,
) -> Result<u64> {
    require!(
        is_adjustable_payment,
        CustomError::AdjustablePaymentDisabled
    );
    resolve_proposal_penalty_bps(dispute_deterrent_enabled)
}

pub fn add_proposal_penalty_bps(current_bps: u64, penalty_bps: u64) -> u64 {
    current_bps
        .saturating_add(penalty_bps)
        .min(MAX_PROPOSAL_PENALTY_BPS)
}

pub fn settlement_fee_discount(mode: SettlementFeeDiscountMode) -> SettlementFeeDiscount {
    match mode {
        SettlementFeeDiscountMode::ConfirmByPayer => SettlementFeeDiscount {
            payer_bps_discount: PAYER_CONFIRM_FEE_REDUCTION_BPS,
            payee_bps_discount: PAYER_CONFIRM_FEE_REDUCTION_BPS,
        },
        SettlementFeeDiscountMode::PayerAcceptPayeeProposal => SettlementFeeDiscount {
            payer_bps_discount: PROPOSAL_COUNTERPARTY_FEE_BPS,
            payee_bps_discount: PROPOSAL_MAKER_FEE_BPS,
        },
        SettlementFeeDiscountMode::PayeeAcceptPayerProposal => SettlementFeeDiscount {
            payer_bps_discount: PROPOSAL_MAKER_FEE_BPS,
            payee_bps_discount: PROPOSAL_COUNTERPARTY_FEE_BPS,
        },
    }
}

/// Payer-initiated cancellations have no fee.
pub fn calc_payer_cancel_fee(_payment_amount: u64) -> Result<u64> {
    Ok(0)
}

/// Calculates confirmation fee from the original payment and splits it between payer and payee.
pub fn calc_confirmation_fee(
    payment_amount: u64,
    accepted_at: i64,
    now: i64,
) -> Result<ConfirmationFee> {
    let elapsed_weeks = calc_elapsed_weeks(accepted_at, now);
    let extra_weeks = elapsed_weeks.saturating_sub(CONFIRMATION_FREE_WEEKS);
    let weekly_bps = extra_weeks
        .checked_mul(CONFIRMATION_WEEKLY_BPS_AFTER_FREE)
        .ok_or(CustomError::AmountOverflow)?;
    let fee_bps = CONFIRMATION_BASE_BPS
        .checked_add(weekly_bps)
        .ok_or(CustomError::AmountOverflow)?;
    let total_fee = calc_fee_amount(payment_amount, fee_bps)?;
    let payer_share = total_fee / 2;
    let payee_share = total_fee
        .checked_sub(payer_share)
        .ok_or(CustomError::AmountUnderflow)?;

    Ok(ConfirmationFee {
        total_fee,
        payer_share,
        payee_share,
    })
}

/// Calculates the payee cancellation fee based on weeks elapsed since acceptance.
pub fn calc_payee_cancel_fee(
    payment_amount: u64,
    accepted_at: i64,
    now: i64,
) -> Result<PayeeCancelFee> {
    let elapsed_weeks = calc_elapsed_weeks(accepted_at, now);
    let weekly_bps = elapsed_weeks
        .checked_mul(PAYEE_CANCEL_WEEKLY_BPS)
        .ok_or(CustomError::AmountOverflow)?;
    let fee_bps = PAYEE_CANCEL_BASE_BPS
        .checked_add(weekly_bps)
        .ok_or(CustomError::AmountOverflow)?;
    let fee_amount = calc_fee_amount(payment_amount, fee_bps)?;

    Ok(PayeeCancelFee {
        fee_amount,
        fee_bps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_penalty_is_zero_when_deterrent_disabled() {
        let bps = resolve_proposal_penalty_bps(false).expect("disabled deterrent should resolve");
        assert_eq!(bps, 0);
    }

    #[test]
    fn proposal_is_rejected_when_order_is_not_adjustable() {
        let result = resolve_order_proposal_penalty_bps(false, true);
        assert!(result.is_err());
    }

    #[test]
    fn order_uses_its_snapshotted_deterrent_setting() {
        let disabled = resolve_order_proposal_penalty_bps(true, false)
            .expect("adjustable order with deterrent disabled should be valid");
        let enabled = resolve_order_proposal_penalty_bps(true, true)
            .expect("adjustable order with deterrent enabled should be valid");

        assert_eq!(disabled, 0);
        assert_eq!(enabled, PROPOSAL_PENALTY_BPS);
    }

    #[test]
    fn proposal_penalty_resolves_when_deterrent_enabled() {
        let bps = resolve_proposal_penalty_bps(true).expect("enabled deterrent should resolve");
        assert_eq!(bps, PROPOSAL_PENALTY_BPS);
        assert!(bps > 0);
    }

    #[test]
    fn add_proposal_penalty_caps_at_max() {
        let near_cap = MAX_PROPOSAL_PENALTY_BPS - 10;
        let capped = add_proposal_penalty_bps(near_cap, PROPOSAL_PENALTY_BPS);
        assert_eq!(capped, MAX_PROPOSAL_PENALTY_BPS);
    }

    #[test]
    fn add_proposal_penalty_never_overflows() {
        let capped = add_proposal_penalty_bps(u64::MAX - 1, PROPOSAL_PENALTY_BPS);
        assert_eq!(capped, MAX_PROPOSAL_PENALTY_BPS);
    }
}
