use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::constants::{SEED_LISTING_ACCOUNT, SEED_LISTING_TOKEN_VAULT_ACCOUNT};
use crate::error::CustomError;
use crate::state::Listing;
use crate::ListingClosed;

pub fn process_close_listing_token(ctx: Context<CloseListingToken>) -> Result<()> {
    let setup = &ctx.accounts.listing;
    require!(
        setup.creator == ctx.accounts.creator.key(),
        CustomError::SetupOnlyCreator
    );
    require!(setup.active_orders == 0, CustomError::SetupNotEmpty);
    require!(
        setup.mint_account == ctx.accounts.mint_account.key(),
        CustomError::SetupMintMismatch
    );

    let remaining_amount = ctx.accounts.listing_token_vault_account.amount;
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_LISTING_ACCOUNT, setup.id.as_ref(), &[setup.bump]]];

    if remaining_amount > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.listing_token_vault_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.creator_token_account.to_account_info(),
                    authority: ctx.accounts.listing.to_account_info(),
                },
                signer_seeds,
            ),
            remaining_amount,
            ctx.accounts.mint_account.decimals,
        )?;
    }

    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.listing_token_vault_account.to_account_info(),
            destination: ctx.accounts.creator.to_account_info(),
            authority: ctx.accounts.listing.to_account_info(),
        },
        signer_seeds,
    ))?;

    let (payer_key, payee_key) = if setup.is_payer_listing {
        (setup.creator, setup.counterparty)
    } else {
        (setup.counterparty, setup.creator)
    };
    emit!(ListingClosed {
        setup: setup.key(),
        creator: ctx.accounts.creator.key(),
        payer: payer_key,
        payee: payee_key,
        mint: ctx.accounts.mint_account.key(),
        mint_decimals: ctx.accounts.mint_account.decimals,
        remaining_amount,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CloseListingToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        close = creator,
        seeds = [SEED_LISTING_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = creator,
        associated_token::token_program = token_program,
    )]
    pub creator_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_LISTING_TOKEN_VAULT_ACCOUNT, listing.id.as_ref()],
        bump = listing.bump_listing_token_vault,
    )]
    pub listing_token_vault_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
