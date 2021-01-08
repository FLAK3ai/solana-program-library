//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the StakePool program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StakePoolError {
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
    /// The program address provided doesn't match the value generated by the program.
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,
    /// The token swap state is invalid.
    #[error("InvalidState")]
    InvalidState,
    /// The calculation failed.
    #[error("CalculationFailure")]
    CalculationFailure,
    /// Stake pool fee > 1.
    #[error("FeeTooHigh")]
    FeeTooHigh,
    /// Token account is associated with the wrong mint.
    #[error("WrongAccountMint")]
    WrongAccountMint,
    /// Account balance should be zero.
    #[error("NonZeroBalance")]
    NonZeroBalance,
    /// Wrong pool owner account.
    #[error("WrongOwner")]
    WrongOwner,
    /// Required signature is missing.
    #[error("SignatureMissing")]
    SignatureMissing,
    /// Invalid validator stake list account.
    #[error("InvalidValidatorStakeList")]
    InvalidValidatorStakeList,
    /// Invalid owner fee account.
    #[error("InvalidFeeAccount")]
    InvalidFeeAccount,
    /// Specified pool mint account is wrong.
    #[error("WrongPoolMint")]
    WrongPoolMint,
    /// Stake account is not in the state expected by the program.
    #[error("WrongStakeState")]
    WrongStakeState,
    /// Stake account voting for this validator already exists in the pool.
    #[error("ValidatorAlreadyAdded")]
    ValidatorAlreadyAdded,
    /// Stake account for this validator not found in the pool.
    #[error("ValidatorNotFound")]
    ValidatorNotFound,
    /// Stake account address not properly derived from the validator address.
    #[error("InvalidStakeAccountAddress")]
    InvalidStakeAccountAddress,
    /// Identify validator stake accounts with old balances and update them.
    #[error("UpdateStakeList")]
    UpdateStakeList,
    /// First udpate old validator stake account balances and then pool stake balance.
    #[error("UpdateStakeListAndPool")]
    UpdateStakeListAndPool,
    /// Validator stake account is not found in the list storage.
    #[error("UnknownValidatorStakeAccount")]
    UnknownValidatorStakeAccount,
}
impl From<StakePoolError> for ProgramError {
    fn from(e: StakePoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for StakePoolError {
    fn type_of() -> &'static str {
        "Stake Pool Error"
    }
}
