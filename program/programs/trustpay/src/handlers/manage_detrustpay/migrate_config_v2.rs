use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

use crate::error::CustomError;
use crate::events::ConfigMigrated;
use crate::state::Config;

use super::MigrateConfigV2;

pub fn process_migrate_config_v2(ctx: Context<MigrateConfigV2>) -> Result<()> {
    let config_info = ctx.accounts.config_account.to_account_info();
    require_keys_eq!(
        *config_info.owner,
        crate::ID,
        CustomError::InvalidConfigAccount
    );

    let migrated_config = {
        let data = config_info.try_borrow_data()?;
        Config::from_legacy_account_data(&data, ctx.bumps.config_account)?
    };

    let authority_key = ctx.accounts.authority.key();
    let upgrade_authority = ctx
        .accounts
        .program_data
        .upgrade_authority_address
        .ok_or(CustomError::UnauthorizedConfigUpdate)?;
    require!(
        authority_key == migrated_config.manage_authority || authority_key == upgrade_authority,
        CustomError::UnauthorizedConfigUpdate
    );

    let required_lamports = Rent::get()?.minimum_balance(Config::ACCOUNT_LEN);
    let current_lamports = config_info.lamports();
    if required_lamports > current_lamports {
        let top_up_lamports = required_lamports
            .checked_sub(current_lamports)
            .ok_or(CustomError::AmountUnderflow)?;
        invoke(
            &system_instruction::transfer(
                &authority_key,
                &ctx.accounts.config_account.key(),
                top_up_lamports,
            ),
            &[
                ctx.accounts.authority.to_account_info(),
                config_info.clone(),
            ],
        )?;
    }

    config_info.resize(Config::ACCOUNT_LEN)?;
    {
        let mut data = config_info.try_borrow_mut_data()?;
        let mut writer = &mut data[..];
        migrated_config.try_serialize(&mut writer)?;
    }

    emit!(ConfigMigrated {
        config: ctx.accounts.config_account.key(),
        authority: authority_key,
        manage_authority: migrated_config.manage_authority,
        fee_vault_account: migrated_config.fee_vault_account,
        previous_version: 1,
        version: migrated_config.version,
        slot: Clock::get()?.slot,
    });

    Ok(())
}
