use anchor_lang::prelude::*;

use crate::events::ConfigUpdated;

use super::{require_manage_authority, UpdateConfigByManageAuthority};

pub fn process_update_enable_custom_deposit(
    ctx: Context<UpdateConfigByManageAuthority>,
    enabled: bool,
) -> Result<()> {
    require_manage_authority(&ctx.accounts.manage_authority, &ctx.accounts.config_account)?;
    let previous_enabled = ctx.accounts.config_account.enable_custom_deposit;
    ctx.accounts.config_account.enable_custom_deposit = enabled;

    emit!(ConfigUpdated {
        field: "enable_custom_deposit".to_string(),
        previous_value: previous_enabled.to_string(),
        new_value: enabled.to_string(),
    });
    Ok(())
}
