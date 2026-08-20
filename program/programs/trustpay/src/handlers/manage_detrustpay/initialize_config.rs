use anchor_lang::prelude::*;

use crate::error::CustomError;
use crate::events::ConfigInitialized;
use crate::state::Config;

use super::InitializeConfig;

pub fn process_initialize_config(
    ctx: Context<InitializeConfig>,
    manage_authority: Pubkey,
    enable_adjustable_payment: bool,
    enable_custom_deposit: bool,
    enable_dispute_deterrent: bool,
) -> Result<()> {
    let upgrade_authority = ctx
        .accounts
        .program_data
        .upgrade_authority_address
        .ok_or(CustomError::UnauthorizedConfigUpdate)?;
    require_keys_eq!(
        upgrade_authority,
        ctx.accounts.authority.key(),
        CustomError::UnauthorizedConfigUpdate
    );

    let slot = Clock::get()?.slot;

    *ctx.accounts.config_account = Config {
        manage_authority,
        bump: ctx.bumps.config_account,
        enable_adjustable_payment,
        enable_custom_deposit,
        enable_dispute_deterrent,
        paused: false,
        paused_at_slot: 0,
        reserved: [0; 64],
    };

    emit!(ConfigInitialized {
        config: ctx.accounts.config_account.key(),
        authority: ctx.accounts.authority.key(),
        manage_authority,
        enable_adjustable_payment,
        enable_custom_deposit,
        enable_dispute_deterrent,
        paused: false,
        slot,
    });

    Ok(())
}
