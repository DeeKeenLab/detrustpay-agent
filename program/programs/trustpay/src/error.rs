use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Amount calculation overflowed")]
    AmountOverflow,
    #[msg("Amount calculation underflowed")]
    AmountUnderflow,
    #[msg("Proposal expiry is required")]
    ProposalExpiryInvalid,
    #[msg("Proposal has expired")]
    ProposalExpired,
    #[msg("No active proposal exists for this responder path")]
    ProposalNotAvailable,
    #[msg("Basic payment deposits must match the payment amount")]
    CounterAmountTooHigh,
    #[msg("Mint account does not match the payment")]
    MintAccountMismatch,
    #[msg("Token vault account does not match the payment")]
    TokenVaultMismatch,
    #[msg("Amount set too high")]
    AmountTooHigh,
    #[msg("Token account does not match the payment")]
    TokenAccountMismatch,
    #[msg("Message exceeds allowed length")]
    MessageTooLong,
    #[msg("Details URL exceeds allowed length")]
    DetailsUrlTooLong,
    #[msg("Version of Order had been updated")]
    OutdatedPaymentVersion,
    #[msg("Order amount must be greater than zero")]
    InvalidPaymentAmount,
    #[msg("Withdrawal amount must be greater than zero")]
    InvalidWithdrawalAmount,
    #[msg("Recipient does not match the provided pubkey")]
    RecipientMismatch,
    #[msg("Adjustable payments are disabled")]
    AdjustablePaymentDisabled,
    #[msg("Custom deposits are disabled")]
    CustomDepositDisabled,
    #[msg("Deposits must equal")]
    DepositMustMatch,
    #[msg("Deposits must equal the payment amount")]
    DepositMustMatchPayment,
    #[msg("Custom deposit must be between 25% and 4x of payment")]
    CustomDepositOutOfRange,
    #[msg("Order id does not match payment account")]
    PaymentIdMismatch,
    #[msg("Invalid setup role")]
    InvalidAcceptCapacity,
    #[msg("Setup counterparty mismatch")]
    SetupCounterpartyMismatch,
    #[msg("Setup capacity exceeded")]
    SetupCapacityExceeded,
    #[msg("Setup vault does not have enough funds")]
    SetupInsufficientFunds,
    #[msg("Setup has active instances")]
    SetupNotEmpty,
    #[msg("Only setup creator can perform this action")]
    SetupOnlyCreator,
    #[msg("Setup account does not match payment")]
    SetupAccountMismatch,
    #[msg("Setup token vault account mismatch")]
    SetupTokenVaultMismatch,
    #[msg("Setup token account mismatch")]
    SetupTokenAccountMismatch,
    #[msg("Setup mint does not match")]
    SetupMintMismatch,
    #[msg("Setup expiration is invalid")]
    InvalidSetupExpiration,
    #[msg("Setup has expired")]
    SetupExpired,
    #[msg("Setup account required")]
    MissingSetupAccount,
    #[msg("Invalid UUID format (expected 32 hex characters, no dashes)")]
    InvalidUuid,
    #[msg("Order is closed")]
    ClosedError,
    #[msg("Order is not closed")]
    PaymentNotClosed,
    #[msg("Order cannot be closed yet")]
    PaymentCloseNotReady,
    #[msg("Only payment creator can perform this action")]
    PaymentOnlyCreator,
    #[msg("Only manage authority can perform this action")]
    UnauthorizedManageAuthority,
    #[msg("Unauthorized config update")]
    UnauthorizedConfigUpdate,
    #[msg("Proposal penalty configuration is invalid")]
    InvalidProposalPenaltyConfig,
    #[msg("Encryption pubkey is required")]
    MissingEncryptionPubkey,
    #[msg("Create listing, accept listing, and direct payment are paused")]
    ProgramPaused,
    #[msg("Buyer and seller order copies do not describe the same order")]
    OrderPairMismatch,
    #[msg("Order copy role does not match its PDA role")]
    OrderRoleMismatch,
    #[msg("Order copy authority does not match its participant")]
    OrderAuthorityMismatch,
    #[msg("Order copy state digest is invalid")]
    OrderDigestMismatch,
    #[msg("Order state could not be serialized")]
    OrderStateSerializationFailed,
    #[msg("Listing creator order-copy rent reserve is insufficient")]
    ListingRentReserveInsufficient,
    #[msg("Only an order participant can perform this action")]
    OrderParticipantOnly,
    #[msg("Order token vault must be closed first")]
    OrderVaultNotClosed,
    #[msg("Order token vault is already closed")]
    OrderVaultAlreadyClosed,
    #[msg("Order token vault must be empty before it can be closed")]
    OrderVaultNotEmpty,
}
