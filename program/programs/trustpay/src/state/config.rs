use anchor_lang::prelude::*;

use crate::error::CustomError;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub manage_authority: Pubkey,
    pub bump: u8,
    pub enable_adjustable_payment: bool,
    pub enable_custom_deposit: bool,
    pub enable_dispute_deterrent: bool,
    pub paused: bool,
    pub paused_at_slot: u64,
    pub reserved: [u8; 64],
}

impl Config {
    pub const ACCOUNT_LEN: usize = 8 + Self::INIT_SPACE;
}

pub fn require_program_active(config: &Config) -> Result<()> {
    require!(!config.paused, CustomError::ProgramPaused);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(paused: bool) -> Config {
        Config {
            manage_authority: Pubkey::new_unique(),
            bump: 1,
            enable_adjustable_payment: true,
            enable_custom_deposit: true,
            enable_dispute_deterrent: true,
            paused,
            paused_at_slot: 0,
            reserved: [0; 64],
        }
    }

    #[test]
    fn active_config_allows_gated_instruction() {
        assert!(require_program_active(&config(false)).is_ok());
    }

    #[test]
    fn paused_config_rejects_gated_instruction() {
        assert!(require_program_active(&config(true)).is_err());
    }

    #[test]
    fn config_uses_the_fresh_unversioned_layout() {
        assert_eq!(Config::ACCOUNT_LEN, 117);
    }
}
