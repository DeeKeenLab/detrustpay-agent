use anchor_lang::prelude::*;

use crate::events::ConfigUpdated;

use super::{require_manage_authority, UpdateConfigByManageAuthority};

pub fn process_update_manage_authority(
    ctx: Context<UpdateConfigByManageAuthority>,
    new_manage_authority: Pubkey,
) -> Result<()> {
    require_manage_authority(&ctx.accounts.manage_authority, &ctx.accounts.config_account)?;

    let previous_manage_authority = ctx.accounts.config_account.manage_authority;
    ctx.accounts.config_account.manage_authority = new_manage_authority;

    emit!(ConfigUpdated {
        field: "manage_authority".to_string(),
        previous_value: previous_manage_authority.to_string(),
        new_value: new_manage_authority.to_string(),
    });

    Ok(())
}
