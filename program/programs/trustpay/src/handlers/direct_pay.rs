use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::SEED_CONFIG_ACCOUNT;
use crate::error::CustomError;
use crate::events::DirectTokenPaid;
use crate::state::{require_fee_vault_account, require_program_active, Config};

pub fn process_direct_pay_token(
    ctx: Context<DirectPayToken>,
    amount: u64,
    payee: Pubkey,
) -> Result<()> {
    let config = &ctx.accounts.config_account;
    require_program_active(config)?;
    require_fee_vault_account(config, &ctx.accounts.fee_vault_account.to_account_info())?;

    require!(amount > 0, CustomError::InvalidPaymentAmount);
    require_keys_eq!(
        ctx.accounts.payee.key(),
        payee,
        CustomError::RecipientMismatch
    );
    let fee = amount
        .checked_div(1_000)
        .ok_or(CustomError::AmountOverflow)?;
    let payee_amount = amount
        .checked_sub(fee)
        .ok_or(CustomError::AmountUnderflow)?;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.payer_token_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.payee_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        payee_amount,
        ctx.accounts.mint_account.decimals,
    )?;
    if fee > 0 {
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            fee,
            ctx.accounts.mint_account.decimals,
        )?;
    }

    emit!(DirectTokenPaid {
        creator: ctx.accounts.payer.key(),
        payer: ctx.accounts.payer.key(),
        payee: ctx.accounts.payee.key(),
        mint: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        amount,
        fee,
        from_account: ctx.accounts.payer_token_account.key(),
        to_account: ctx.accounts.payee_token_account.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct DirectPayToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Recipient pubkey is validated against the provided argument.
    pub payee: UncheckedAccount<'info>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub payer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payee,
        associated_token::token_program = token_program,
    )]
    pub payee_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,

    /// CHECK: Account is validated against config in instruction logic.
    pub fee_vault_account: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = fee_vault_account,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
