use anchor_lang::prelude::*;

use crate::error::CustomError;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub manage_authority: Pubkey,
    pub fee_vault_account: Pubkey,
    pub bump: u8,
    pub enable_adjustable_payment: bool,
    pub enable_custom_deposit: bool,
    pub enable_dispute_deterrent: bool,
    pub paused: bool,
    pub version: u8,
    pub paused_at_slot: u64,
    pub reserved: [u8; 32],
}

impl Config {
    pub const VERSION: u8 = 2;
    pub const ACCOUNT_LEN: usize = 8 + Self::INIT_SPACE;
    pub const LEGACY_ACCOUNT_LEN: usize = 8 + 32 + 32 + 1 + 1 + 1 + 1;

    pub fn from_legacy_account_data(data: &[u8], expected_bump: u8) -> Result<Self> {
        require!(
            data.len() == Self::LEGACY_ACCOUNT_LEN,
            CustomError::InvalidConfigAccount
        );
        require!(
            data.len() >= Self::DISCRIMINATOR.len()
                && &data[..Self::DISCRIMINATOR.len()] == Self::DISCRIMINATOR,
            CustomError::InvalidConfigAccount
        );

        let manage_authority = read_pubkey(data, 8)?;
        let fee_vault_account = read_pubkey(data, 40)?;
        let bump = data[72];
        require_eq!(bump, expected_bump, CustomError::InvalidConfigAccount);

        Ok(Self {
            manage_authority,
            fee_vault_account,
            bump,
            enable_adjustable_payment: data[73] != 0,
            enable_custom_deposit: data[74] != 0,
            enable_dispute_deterrent: data[75] != 0,
            paused: false,
            version: Self::VERSION,
            paused_at_slot: 0,
            reserved: [0; 32],
        })
    }
}

fn read_pubkey(data: &[u8], offset: usize) -> Result<Pubkey> {
    let end = offset
        .checked_add(32)
        .ok_or(error!(CustomError::InvalidConfigAccount))?;
    let bytes: [u8; 32] = data
        .get(offset..end)
        .ok_or(error!(CustomError::InvalidConfigAccount))?
        .try_into()
        .map_err(|_| error!(CustomError::InvalidConfigAccount))?;
    Ok(Pubkey::new_from_array(bytes))
}

pub fn require_fee_vault_account(config: &Config, fee_vault_info: &AccountInfo) -> Result<()> {
    require!(
        config.fee_vault_account != Pubkey::default(),
        CustomError::InvalidConfigAccount
    );
    require_keys_eq!(
        config.fee_vault_account,
        fee_vault_info.key(),
        CustomError::InvalidConfigAccount
    );

    Ok(())
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
            fee_vault_account: Pubkey::new_unique(),
            bump: 1,
            enable_adjustable_payment: true,
            enable_custom_deposit: true,
            enable_dispute_deterrent: true,
            paused,
            version: Config::VERSION,
            paused_at_slot: 0,
            reserved: [0; 32],
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
}
