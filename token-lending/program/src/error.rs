//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the TokenLending program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum LendingError {
    // 0
    /// Invalid instruction data passed in.
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,
    /// The account cannot be initialized because it is already in use.
    #[error("Account is already initialized")]
    AlreadyInitialized,
    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    /// The program address provided doesn't match the value generated by the program.
    #[error("Market authority is invalid")]
    InvalidMarketAuthority,
    /// Expected a different market owner
    #[error("Market owner is invalid")]
    InvalidMarketOwner,

    // 5
    /// The owner of the input isn't set to the program address generated by the program.
    #[error("Input account owner is not the program address")]
    InvalidAccountOwner,
    /// The owner of the account input isn't set to the correct token program id.
    #[error("Input token account is not owned by the correct token program id")]
    InvalidTokenOwner,
    /// Expected an SPL Token mint
    #[error("Input token mint account is not valid")]
    InvalidTokenMint,
    /// Expected a different SPL Token program
    #[error("Input token program account is not valid")]
    InvalidTokenProgram,
    /// Invalid amount, must be greater than zero
    #[error("Input amount is invalid")]
    InvalidAmount,

    // 10
    /// Invalid config value
    #[error("Input config value is invalid")]
    InvalidConfig,
    /// Invalid config value
    #[error("Input account must be a signer")]
    InvalidSigner,
    /// Invalid account input
    #[error("Invalid account input")]
    InvalidAccountInput,
    /// Math operation overflow
    #[error("Math operation overflow")]
    MathOverflow,
    /// Negative interest rate
    #[error("Interest rate is negative")]
    NegativeInterestRate,

    // 15
    /// Token initialize mint failed
    #[error("Token initialize mint failed")]
    TokenInitializeMintFailed,
    /// Token initialize account failed
    #[error("Token initialize account failed")]
    TokenInitializeAccountFailed,
    /// Token transfer failed
    #[error("Token transfer failed")]
    TokenTransferFailed,
    /// Token mint to failed
    #[error("Token mint to failed")]
    TokenMintToFailed,
    /// Token burn failed
    #[error("Token burn failed")]
    TokenBurnFailed,

    // 20
    // @TODO: this is only used in one place that might be removed.
    /// Input reserves cannot be the same
    #[error("Input reserves cannot be the same")]
    DuplicateReserve,
    // @TODO: this is only used in one place that might be removed.
    /// Input reserves cannot use the same liquidity mint
    #[error("Input reserves cannot use the same liquidity mint")]
    DuplicateReserveMint,
    /// Insufficient liquidity available
    #[error("Insufficient liquidity available")]
    InsufficientLiquidity,
    /// This reserve's collateral cannot be used for borrows
    #[error("Input reserve has collateral disabled")]
    ReserveCollateralDisabled,
    /// Reserve state stale
    #[error("Reserve state needs to be refreshed")]
    ReserveStale,

    // 25
    /// Withdraw amount too small
    #[error("Withdraw amount too small")]
    WithdrawTooSmall,
    /// Withdraw amount too large
    #[error("Withdraw amount too large")]
    WithdrawTooLarge,
    /// Borrow amount too small
    #[error("Borrow amount too small to receive liquidity after fees")]
    BorrowTooSmall,
    /// Borrow amount too large
    #[error("Borrow amount too large for deposited collateral")]
    BorrowTooLarge,
    /// Repay amount too small
    #[error("Repay amount too small to transfer liquidity")]
    RepayTooSmall,

    // 30
    /// Liquidation amount too small
    #[error("Liquidation amount too small to receive collateral")]
    LiquidationTooSmall,
    /// Cannot liquidate healthy obligations
    #[error("Cannot liquidate healthy obligations")]
    ObligationHealthy,
    /// Obligation state stale
    #[error("Obligation state needs to be refreshed")]
    ObligationStale,
    /// Obligation reserve limit exceeded
    #[error("Obligation reserve limit exceeded")]
    ObligationReserveLimit,
    /// Obligation loan to value limit exceeded
    #[error("Obligation loan to value limit exceeded")]
    ObligationLoanToValueLimit,

    // 35
    /// Expected a different obligation owner
    #[error("Obligation owner is invalid")]
    InvalidObligationOwner,
    /// Invalid obligation collateral
    #[error("Invalid obligation collateral")]
    InvalidObligationCollateral,
    /// Invalid obligation liquidity
    #[error("Invalid obligation liquidity")]
    InvalidObligationLiquidity,
    /// Obligation collateral is empty
    #[error("Obligation collateral is empty")]
    ObligationCollateralEmpty,
    /// Obligation liquidity is empty
    #[error("Obligation liquidity is empty")]
    ObligationLiquidityEmpty,
}

impl From<LendingError> for ProgramError {
    fn from(e: LendingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for LendingError {
    fn type_of() -> &'static str {
        "Lending Error"
    }
}
