use anchor_lang::prelude::*;

use crate::constants::SEED_CONFIG_ACCOUNT;
use crate::error::CustomError;
use crate::state::Config;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, crate::program::Detrustpay>,

    #[account(constraint = program_data.upgrade_authority_address.is_some())]
    pub program_data: Account<'info, ProgramData>,

    #[account(
        init,
        payer = authority,
        space = Config::ACCOUNT_LEN,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfigByManageAuthority<'info> {
    pub manage_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
}

pub fn require_manage_authority(
    manage_authority: &Signer,
    config_account: &Account<Config>,
) -> Result<()> {
    require_keys_eq!(
        manage_authority.key(),
        config_account.manage_authority,
        CustomError::UnauthorizedManageAuthority
    );

    Ok(())
}

pub mod initialize_config;
pub use initialize_config::*;

pub mod update_enable_adjustable_payment;
pub use update_enable_adjustable_payment::*;

pub mod update_enable_custom_deposit;
pub use update_enable_custom_deposit::*;

pub mod update_enable_dispute_deterrent;
pub use update_enable_dispute_deterrent::*;

pub mod withdraw_protocol_fees;
pub use withdraw_protocol_fees::*;

pub mod update_manage_authority;
pub use update_manage_authority::*;

pub mod update_program_paused;
pub use update_program_paused::*;
