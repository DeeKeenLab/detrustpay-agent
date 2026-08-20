use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::{SEED_CONFIG_ACCOUNT, SEED_PROTOCOL_FEE_VAULT_ACCOUNT};
use crate::error::CustomError;
use crate::events::ProtocolFeesWithdrawn;
use crate::state::Config;

use super::require_manage_authority;

pub fn process_withdraw_protocol_fees(
    ctx: Context<WithdrawProtocolFees>,
    amount: u64,
) -> Result<()> {
    require_manage_authority(&ctx.accounts.manage_authority, &ctx.accounts.config_account)?;
    require!(amount > 0, CustomError::InvalidWithdrawalAmount);
    require!(
        amount <= ctx.accounts.protocol_fee_vault.amount,
        CustomError::AmountTooHigh
    );

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_CONFIG_ACCOUNT, &[ctx.accounts.config_account.bump]]];

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.protocol_fee_vault.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.destination_token_account.to_account_info(),
                authority: ctx.accounts.config_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.mint_account.decimals,
    )?;

    emit!(ProtocolFeesWithdrawn {
        authority: ctx.accounts.manage_authority.key(),
        mint: ctx.accounts.mint_account.key(),
        protocol_fee_vault: ctx.accounts.protocol_fee_vault.key(),
        destination_token_account: ctx.accounts.destination_token_account.key(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawProtocolFees<'info> {
    pub manage_authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_PROTOCOL_FEE_VAULT_ACCOUNT, mint_account.key().as_ref()],
        bump,
        token::mint = mint_account,
        token::authority = config_account,
        token::token_program = token_program,
    )]
    pub protocol_fee_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint_account,
        token::token_program = token_program,
    )]
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_fee_vault_is_stable_and_mint_specific() {
        let first_mint = Pubkey::new_unique();
        let second_mint = Pubkey::new_unique();
        let (first_vault, _) = Pubkey::find_program_address(
            &[SEED_PROTOCOL_FEE_VAULT_ACCOUNT, first_mint.as_ref()],
            &crate::ID,
        );
        let (same_first_vault, _) = Pubkey::find_program_address(
            &[SEED_PROTOCOL_FEE_VAULT_ACCOUNT, first_mint.as_ref()],
            &crate::ID,
        );
        let (second_vault, _) = Pubkey::find_program_address(
            &[SEED_PROTOCOL_FEE_VAULT_ACCOUNT, second_mint.as_ref()],
            &crate::ID,
        );

        assert_eq!(first_vault, same_first_vault);
        assert_ne!(first_vault, second_vault);
    }
}
