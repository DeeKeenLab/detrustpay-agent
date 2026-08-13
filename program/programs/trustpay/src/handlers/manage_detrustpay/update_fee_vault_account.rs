use anchor_lang::prelude::*;

use crate::events::ConfigUpdated;

use super::{require_manage_authority, UpdateFeeVaultAccountByManageAuthority};

pub fn process_update_fee_vault_account(
    ctx: Context<UpdateFeeVaultAccountByManageAuthority>,
) -> Result<()> {
    require_manage_authority(&ctx.accounts.manage_authority, &ctx.accounts.config_account)?;

    let previous_fee_vault_account = ctx.accounts.config_account.fee_vault_account;

    ctx.accounts.config_account.fee_vault_account = ctx.accounts.fee_vault_account.key();

    emit!(ConfigUpdated {
        field: "fee_vault_account".to_string(),
        previous_value: previous_fee_vault_account.to_string(),
        new_value: ctx.accounts.fee_vault_account.key().to_string(),
    });

    Ok(())
}
