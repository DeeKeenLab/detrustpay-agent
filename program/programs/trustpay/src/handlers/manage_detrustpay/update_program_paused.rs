use anchor_lang::prelude::*;

use crate::events::ProgramPauseUpdated;

use super::{require_manage_authority, UpdateConfigByManageAuthority};

pub fn process_update_program_paused(
    ctx: Context<UpdateConfigByManageAuthority>,
    paused: bool,
) -> Result<()> {
    require_manage_authority(&ctx.accounts.manage_authority, &ctx.accounts.config_account)?;

    let slot = Clock::get()?.slot;
    let previous_paused = ctx.accounts.config_account.paused;
    ctx.accounts.config_account.paused = paused;
    ctx.accounts.config_account.paused_at_slot = if paused { slot } else { 0 };

    emit!(ProgramPauseUpdated {
        authority: ctx.accounts.manage_authority.key(),
        previous_paused,
        paused,
        slot,
    });

    Ok(())
}
