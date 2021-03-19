//! Instruction types

use crate::{
    error::LendingError,
    state::{ReserveConfig, ReserveFees},
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    sysvar,
};
use std::{convert::TryInto, mem::size_of};

/// Describe how an input amount should be treated
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum AmountType {
    /// Treat amount as an exact amount of tokens
    ExactAmount,
    /// Treat amount as a percentage of available tokens
    PercentAmount,
}

/// Instructions supported by the lending program.
#[derive(Clone, Debug, PartialEq)]
pub enum LendingInstruction {
    // 0
    /// Initializes a new lending market.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Lending market account
    ///   1. `[]` Quote currency SPL Token mint
    ///   2. `[]` Rent sysvar
    ///   3. `[]` Token program id
    InitLendingMarket {
        /// Owner authority which can add new reserves
        owner: Pubkey,
        /// The ratio of the loan to the value of the collateral as a percent
        loan_to_value_ratio: u8,
        /// The percent at which an obligation is considered unhealthy
        liquidation_threshold: u8,
    },

    // 1
    /// Sets the new owner of a lending market.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Lending market account
    ///   1. `[signer]` Current owner
    SetLendingMarketOwner {
        /// The new owner
        new_owner: Pubkey,
    },

    // 2
    /// Initializes a new lending market reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account - uninitialized
    ///   2. `[writable]` Reserve account
    ///   3. `[]` Reserve liquidity SPL Token mint
    ///   4. `[writable]` Reserve liquidity supply SPL Token account - uninitialized
    ///   7. `[writable]` Reserve liquidity fee receiver - uninitialized
    ///   5. `[writable]` Reserve collateral SPL Token mint - uninitialized
    ///   6. `[writable]` Reserve collateral token supply - uninitialized
    ///   8. `[]` Lending market account
    ///   9. `[signer]` Lending market owner
    ///   10 `[]` Derived lending market authority
    ///   11 `[]` User transfer authority ($authority)
    ///   12 `[]` Clock sysvar
    ///   13 `[]` Rent sysvar
    ///   14 `[]` Token program id
    ///   15 `[optional]` Flux Aggregator oracle account
    ///                     Not required for quote currency reserves.
    ///                     Must match quote and base currency.
    InitReserve {
        /// Initial amount of liquidity to deposit into the new reserve
        liquidity_amount: u64,
        /// Reserve configuration values
        config: ReserveConfig,
    },

    // 3
    /// Deposit liquidity into a reserve in exchange for collateral representing ownership of the
    /// reserve liquidity pool.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account
    ///   2. `[writable]` Reserve account
    ///   3. `[writable]` Reserve liquidity supply SPL Token account
    ///   4. `[writable]` Reserve collateral SPL Token mint
    ///   5. `[]` Lending market account
    ///   6. `[]` Derived lending market authority
    ///   7. `[signer]` User transfer authority ($authority)
    ///   8. `[]` Clock sysvar
    ///   9. `[]` Token program id
    DepositReserveLiquidity {
        /// Amount of liquidity to deposit in exchange for collateral
        liquidity_amount: u64,
    },

    // 4
    /// Redeem collateral from a reserve in exchange for liquidity.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source collateral token account
    ///                     $authority can transfer $collateral_amount.
    ///   1. `[writable]` Destination liquidity token account
    ///   2. `[writable]` Reserve account
    ///   3. `[writable]` Reserve collateral SPL Token mint
    ///   4. `[writable]` Reserve liquidity supply SPL Token account
    ///   5. `[]` Lending market account
    ///   6. `[]` Derived lending market authority
    ///   7. `[signer]` User transfer authority ($authority)
    ///   8. `[]` Token program id
    RedeemReserveCollateral {
        /// Amount of collateral to return in exchange for liquidity
        collateral_amount: u64,
    },

    // 5
    /// Accrue interest on reserves
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` Clock sysvar
    ///   1. `[writable]` Reserve account
    ///   .. `[writable]` Additional reserve accounts
    AccrueReserveInterest,

    // 6
    /// Initializes a new lending market obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation account - uninitialized
    ///   1. `[]` Lending market account
    ///   2. `[]` Clock sysvar
    ///   3. `[]` Rent sysvar
    ///   4. `[]` Token program id
    InitObligation,

    // 7
    /// Refresh an obligation's loan to value ratio.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation account
    ///   1. `[]` Lending market account
    ///   2. `[]` Clock sysvar
    ///   3. `[]` Token program id
    ///   4..4+N `[]` Obligation collateral and liquidity accounts
    ///                 Must be all initialized collateral accounts in exact order, followed by
    ///                 all initialized liquidity accounts in exact order, with no additional
    ///                 accounts following.
    RefreshObligation,

    // 8
    /// Initializes a new obligation collateral.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation account
    ///   1. `[writable]` Obligation collateral account - uninitialized
    ///   2. `[]` Deposit reserve account
    ///   3. `[writable]` Obligation token mint - uninitialized
    ///   4. `[writable]` Obligation token output account
    ///   5. `[]` Obligation token owner
    ///   6. `[]` Lending market account
    ///   7. `[]` Derived lending market authority
    ///   8. `[]` Clock sysvar
    ///   9. `[]` Rent sysvar
    ///   10 `[]` Token program id
    InitObligationCollateral,

    // 9
    /// Refresh market value of an obligation's collateral.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation collateral account
    ///   1. `[]` Deposit reserve account
    ///   2. `[]` Lending market account
    ///   3. `[]` Derived lending market authority
    ///   4. `[]` Dex market
    ///   5. `[]` Dex market order book side
    ///   6. `[]` Temporary memory
    ///   7. `[]` Clock sysvar
    ///   8. `[]` Token program id
    RefreshObligationCollateral,

    // 10
    /// Deposit collateral to an obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source collateral token account
    ///                     Minted by deposit reserve collateral mint.
    ///                     $authority can transfer $collateral_amount.
    ///   1. `[writable]` Destination deposit reserve collateral supply SPL Token account
    ///   2. `[]` Deposit reserve account
    ///   3. `[writable]` Obligation account
    ///   4. `[writable]` Obligation collateral account
    ///   5. `[writable]` Obligation token mint
    ///   6. `[writable]` Obligation token output account
    ///   7. `[]` Lending market account
    ///   8. `[]` Derived lending market authority
    ///   9. `[signer]` User transfer authority ($authority)
    ///   10 `[]` Token program id
    DepositObligationCollateral {
        /// Amount of collateral to deposit
        collateral_amount: u64,
    },

    // 11
    /// Withdraw collateral from an obligation. The loan must remain healthy. Requires a
    /// recently refreshed obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source withdraw reserve collateral supply SPL Token account
    ///   1. `[writable]` Destination collateral token account
    ///                     Minted by withdraw reserve collateral mint.
    ///                     $authority can transfer $collateral_amount.
    ///   2. `[]` Withdraw reserve account
    ///   3. `[writable]` Obligation account
    ///   4. `[writable]` Obligation collateral account
    ///   5. `[writable]` Obligation token mint
    ///   6. `[writable]` Obligation token input account
    ///   7. `[]` Lending market account
    ///   8. `[]` Derived lending market authority
    ///   9. `[signer]` User transfer authority ($authority)
    ///   10 `[]` Clock sysvar
    ///   11 `[]` Token program id
    WithdrawObligationCollateral {
        /// Amount of collateral to withdraw - usage depends on `collateral_amount_type`
        collateral_amount: u64,
        /// Describe how `collateral_amount` should be treated
        collateral_amount_type: AmountType,
    },

    // 12
    /// Initializes a new obligation liquidity.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation account
    ///   1. `[writable]` Obligation liquidity account - uninitialized
    ///   2. `[]` Borrow reserve account
    ///   3. `[]` Lending market account
    ///   4. `[]` Clock sysvar
    ///   5. `[]` Rent sysvar
    ///   6. `[]` Token program id
    InitObligationLiquidity,

    // 13
    /// Refresh market value and accrued interest of an obligation's liquidity.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation liquidity account
    ///   1. `[]` Borrow reserve account
    ///   2. `[]` Lending market account
    ///   3. `[]` Derived lending market authority
    ///   4. `[]` Dex market
    ///   5. `[]` Dex market order book side
    ///   6. `[]` Temporary memory
    ///   7. `[]` Clock sysvar
    ///   8. `[]` Token program id
    RefreshObligationLiquidity,

    // 14
    /// Borrow liquidity from a reserve by depositing collateral tokens. The amount of liquidity is
    /// determined by market price. Requires a recently refreshed obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source borrow reserve liquidity supply SPL Token account
    ///   1. `[writable]` Destination liquidity token account
    ///                     Minted by borrow reserve liquidity mint.
    ///   2. `[writable]` Borrow reserve account
    ///   3. `[writable]` Borrow reserve liquidity fee receiver account
    ///                     Must be the fee account specified at InitReserve.
    ///   4. `[writable]` Obligation account
    ///   5. `[writable]` Obligation liquidity account
    ///   6. `[]` Lending market account
    ///   7. `[]` Derived lending market authority
    ///   8. `[]` Dex market
    ///   9. `[]` Dex market order book side
    ///   10 `[]` Temporary memory
    ///   11 `[]` Clock sysvar
    ///   12 `[]` Token program id
    ///   13 `[optional, writable]` Host fee receiver account
    BorrowObligationLiquidity {
        /// Amount of liquidity to borrow - usage depends on `liquidity_amount_type`
        liquidity_amount: u64,
        /// Describe how `liquidity_amount` should be treated
        liquidity_amount_type: AmountType,
        // TODO: slippage constraint - https://git.io/JmV67
    },

    // 15
    /// Repay borrowed liquidity to a reserve. Requires a recently refreshed obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account
    ///                     Minted by repay reserve liquidity mint.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination repay reserve liquidity supply SPL Token account
    ///   2. `[writable]` Repay reserve account
    ///   3. `[writable]` Obligation account
    ///   4. `[writable]` Obligation liquidity account
    ///   5. `[]` Lending market account
    ///   6. `[]` Derived lending market authority
    ///   7. `[signer]` User transfer authority ($authority)
    ///   8. `[]` Clock sysvar
    ///   9. `[]` Token program id
    RepayObligationLiquidity {
        /// Amount of liquidity to repay - usage depends on `liquidity_amount_type`
        liquidity_amount: u64,
        /// Describe how `liquidity_amount` should be treated
        liquidity_amount_type: AmountType,
    },

    // 16
    /// Repay borrowed liquidity to a reserve to receive collateral at a discount from an unhealthy
    /// obligation. Requires a recently refreshed obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account
    ///                     Minted by repay reserve liquidity mint.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account
    ///                     Minted by withdraw reserve collateral mint
    ///   2. `[writable]` Repay reserve account
    ///   3. `[writable]` Repay reserve liquidity supply SPL Token account
    ///   4. `[writable]` Withdraw reserve account
    ///   5. `[writable]` Withdraw reserve collateral supply SPL Token account
    ///   6. `[writable]` Obligation account
    ///   7. `[writable]` Obligation liquidity account
    ///   8. `[writable]` Obligation collateral account
    ///   9. `[]` Lending market account
    ///   10 `[]` Derived lending market authority
    ///   11 `[signer]` User transfer authority ($authority)
    ///   12 `[]` Clock sysvar
    ///   13 `[]` Token program id
    LiquidateObligation {
        /// Amount of liquidity to repay - usage depends on `liquidity_amount_type`
        liquidity_amount: u64,
        /// Describe how `liquidity_amount` should be treated
        liquidity_amount_type: AmountType,
    },
}

impl LendingInstruction {
    /// Unpacks a byte buffer into a [LendingInstruction](enum.LendingInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(LendingError::InstructionUnpackError)?;
        Ok(match tag {
            0 => {
                let (owner, _rest) = Self::unpack_pubkey(rest)?;
                let (loan_to_value_ratio, rest) = Self::unpack_u8(rest)?;
                let (liquidation_threshold, rest) = Self::unpack_u8(rest)?;
                Self::InitLendingMarket {
                    owner,
                    loan_to_value_ratio,
                    liquidation_threshold,
                }
            }
            1 => {
                let (new_owner, _rest) = Self::unpack_pubkey(rest)?;
                Self::SetLendingMarketOwner { new_owner }
            }
            2 => {
                let (liquidity_amount, rest) = Self::unpack_u64(rest)?;
                let (optimal_utilization_rate, rest) = Self::unpack_u8(rest)?;
                let (liquidation_bonus, rest) = Self::unpack_u8(rest)?;
                let (min_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (optimal_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (max_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (borrow_fee_wad, rest) = Self::unpack_u64(rest)?;
                let (host_fee_percentage, _rest) = Self::unpack_u8(rest)?;
                Self::InitReserve {
                    liquidity_amount,
                    config: ReserveConfig {
                        optimal_utilization_rate,
                        liquidation_bonus,
                        min_borrow_rate,
                        optimal_borrow_rate,
                        max_borrow_rate,
                        fees: ReserveFees {
                            borrow_fee_wad,
                            host_fee_percentage,
                        },
                    },
                }
            }
            3 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositReserveLiquidity { liquidity_amount }
            }
            4 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                Self::RedeemReserveCollateral { collateral_amount }
            }
            5 => Self::AccrueReserveInterest,
            6 => Self::InitObligation,
            7 => Self::RefreshObligation,
            8 => Self::InitObligationCollateral,
            9 => Self::RefreshObligationCollateral,
            10 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositObligationCollateral { collateral_amount }
            }
            11 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                let (collateral_amount_type, _rest) = Self::unpack_u8(rest)?;
                let collateral_amount_type = AmountType::from_u8(collateral_amount_type)
                    .ok_or(LendingError::InstructionUnpackError)?;
                Self::WithdrawObligationCollateral {
                    collateral_amount,
                    collateral_amount_type,
                }
            }
            12 => Self::InitObligationLiquidity,
            13 => Self::RefreshObligationLiquidity,
            14 => {
                let (liquidity_amount, rest) = Self::unpack_u64(rest)?;
                let (liquidity_amount_type, _rest) = Self::unpack_u8(rest)?;
                let liquidity_amount_type = AmountType::from_u8(liquidity_amount_type)
                    .ok_or(LendingError::InstructionUnpackError)?;
                Self::BorrowObligationLiquidity {
                    liquidity_amount,
                    liquidity_amount_type,
                }
            }
            15 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                let (liquidity_amount_type, _rest) = Self::unpack_u8(rest)?;
                let liquidity_amount_type = AmountType::from_u8(liquidity_amount_type)
                    .ok_or(LendingError::InstructionUnpackError)?;
                Self::RepayObligationLiquidity {
                    liquidity_amount,
                    liquidity_amount_type,
                }
            }
            16 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                let (liquidity_amount_type, _rest) = Self::unpack_u8(rest)?;
                let liquidity_amount_type = AmountType::from_u8(liquidity_amount_type)
                    .ok_or(LendingError::InstructionUnpackError)?;
                Self::LiquidateObligation {
                    liquidity_amount,
                    liquidity_amount_type,
                }
            }
            _ => return Err(LendingError::InstructionUnpackError.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(LendingError::InstructionUnpackError)?;
            Ok((amount, rest))
        } else {
            Err(LendingError::InstructionUnpackError.into())
        }
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if !input.is_empty() {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(LendingError::InstructionUnpackError)?;
            Ok((amount, rest))
        } else {
            Err(LendingError::InstructionUnpackError.into())
        }
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() >= PUBKEY_BYTES {
            let (key, rest) = input.split_at(PUBKEY_BYTES);
            let pk = Pubkey::new(key);
            Ok((pk, rest))
        } else {
            Err(LendingError::InstructionUnpackError.into())
        }
    }

    /// Packs a [LendingInstruction](enum.LendingInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match *self {
            Self::InitLendingMarket {
                owner,
                loan_to_value_ratio,
                liquidation_threshold,
            } => {
                buf.push(0);
                buf.extend_from_slice(owner.as_ref());
                buf.extend_from_slice(&loan_to_value_ratio.to_le_bytes());
                buf.extend_from_slice(&liquidation_threshold.to_le_bytes());
            }
            Self::SetLendingMarketOwner { new_owner } => {
                buf.push(1);
                buf.extend_from_slice(new_owner.as_ref());
            }
            Self::InitReserve {
                liquidity_amount,
                config:
                    ReserveConfig {
                        optimal_utilization_rate,
                        liquidation_bonus,
                        min_borrow_rate,
                        optimal_borrow_rate,
                        max_borrow_rate,
                        fees:
                            ReserveFees {
                                borrow_fee_wad,
                                host_fee_percentage,
                            },
                    },
            } => {
                buf.push(2);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&optimal_utilization_rate.to_le_bytes());
                buf.extend_from_slice(&liquidation_bonus.to_le_bytes());
                buf.extend_from_slice(&min_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&optimal_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&max_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&borrow_fee_wad.to_le_bytes());
                buf.extend_from_slice(&host_fee_percentage.to_le_bytes());
            }
            Self::DepositReserveLiquidity { liquidity_amount } => {
                buf.push(3);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::RedeemReserveCollateral { collateral_amount } => {
                buf.push(4);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::AccrueReserveInterest => {
                buf.push(5);
            }
            Self::InitObligation => {
                buf.push(6);
            }
            Self::RefreshObligation => {
                buf.push(7);
            }
            Self::InitObligationCollateral => {
                buf.push(8);
            }
            Self::RefreshObligationCollateral => {
                buf.push(9);
            }
            Self::DepositObligationCollateral { collateral_amount } => {
                buf.push(10);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::WithdrawObligationCollateral {
                collateral_amount,
                collateral_amount_type,
            } => {
                buf.push(11);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
                buf.extend_from_slice(&collateral_amount_type.to_u8().unwrap().to_le_bytes());
            }
            Self::InitObligationLiquidity => {
                buf.push(12);
            }
            Self::RefreshObligationLiquidity => {
                buf.push(13);
            }
            Self::BorrowObligationLiquidity {
                liquidity_amount,
                liquidity_amount_type,
            } => {
                buf.push(14);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&liquidity_amount_type.to_u8().unwrap().to_le_bytes());
            }
            Self::RepayObligationLiquidity {
                liquidity_amount,
                liquidity_amount_type,
            } => {
                buf.push(15);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&liquidity_amount_type.to_u8().unwrap().to_le_bytes());
            }
            Self::LiquidateObligation {
                liquidity_amount,
                liquidity_amount_type,
            } => {
                buf.push(16);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&liquidity_amount_type.to_u8().unwrap().to_le_bytes());
            }
        }
        buf
    }
}

/// Creates an 'InitLendingMarket' instruction.
pub fn init_lending_market(
    program_id: Pubkey,
    loan_to_value_ratio: u8,
    liquidation_threshold: u8,
    lending_market_pubkey: Pubkey,
    lending_market_owner: Pubkey,
    quote_token_mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(quote_token_mint, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::InitLendingMarket {
            owner: lending_market_owner,
            loan_to_value_ratio,
            liquidation_threshold,
        }
        .pack(),
    }
}

/// Creates a 'SetLendingMarketOwner' instruction.
pub fn set_lending_market_owner(
    program_id: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner: Pubkey,
    new_owner: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_owner, true),
        ],
        data: LendingInstruction::SetLendingMarketOwner { new_owner }.pack(),
    }
}

/// Creates an 'InitReserve' instruction.
#[allow(clippy::too_many_arguments)]
pub fn init_reserve(
    program_id: Pubkey,
    liquidity_amount: u64,
    config: ReserveConfig,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_mint_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    reserve_liquidity_fee_receiver_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    reserve_collateral_supply_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
    reserve_liquidity_aggregator_pubkey: Option<Pubkey>,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    let mut accounts = vec![
        AccountMeta::new(source_liquidity_pubkey, false),
        AccountMeta::new(destination_collateral_pubkey, false),
        AccountMeta::new(reserve_pubkey, false),
        AccountMeta::new_readonly(reserve_liquidity_mint_pubkey, false),
        AccountMeta::new(reserve_liquidity_supply_pubkey, false),
        AccountMeta::new(reserve_liquidity_fee_receiver_pubkey, false),
        AccountMeta::new(reserve_collateral_mint_pubkey, false),
        AccountMeta::new(reserve_collateral_supply_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_owner_pubkey, true),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    if let Some(reserve_liquidity_aggregator_pubkey) = reserve_liquidity_aggregator_pubkey {
        accounts.push(AccountMeta::new_readonly(reserve_liquidity_aggregator_pubkey, false));
    }
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::InitReserve {
            liquidity_amount,
            config,
        }
        .pack(),
    }
}

/// Creates a 'DepositReserveLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn deposit_reserve_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::DepositReserveLiquidity { liquidity_amount }.pack(),
    }
}

/// Creates a 'RedeemReserveCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn redeem_reserve_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::RedeemReserveCollateral { collateral_amount }.pack(),
    }
}

/// Creates an `AccrueReserveInterest` instruction
pub fn accrue_reserve_interest(program_id: Pubkey, reserve_pubkeys: Vec<Pubkey>) -> Instruction {
    let mut accounts = Vec::with_capacity(1 + reserve_pubkeys.len());
    accounts.push(AccountMeta::new_readonly(sysvar::clock::id(), false));
    accounts.extend(
        reserve_pubkeys
            .into_iter()
            .map(|reserve_pubkey| AccountMeta::new(reserve_pubkey, false)),
    );
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::AccrueReserveInterest.pack(),
    }
}

/// Creates an 'InitObligation' instruction.
#[allow(clippy::too_many_arguments)]
pub fn init_obligation(
    program_id: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::InitObligation.pack(),
    }
}

/// Creates a 'RefreshObligation' instruction.
#[allow(clippy::too_many_arguments)]
pub fn refresh_obligation(
    program_id: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    obligation_collateral_liquidity_pubkeys: Vec<Pubkey>,
) -> Instruction {
    let mut accounts = Vec::with_capacity(4 + obligation_collateral_liquidity_pubkeys.len());
    accounts.extend(vec![
        AccountMeta::new(obligation_pubkey, false)
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ]);
    accounts.extend(
        obligation_collateral_liquidity_pubkeys
            .into_iter()
            .map(|pubkey| AccountMeta::new_readonly(pubkey, false)),
    );
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::RefreshObligationLiquidity.pack(),
    }
}

/// Creates an 'InitObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn init_obligation_collateral(
    program_id: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_collateral_pubkey: Pubkey,
    deposit_reserve_pubkey: Pubkey,
    obligation_token_mint_pubkey: Pubkey,
    obligation_token_output_pubkey: Pubkey,
    obligation_token_owner_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_collateral_pubkey, false),
            AccountMeta::new_readonly(deposit_reserve_pubkey, false),
            AccountMeta::new(obligation_token_mint_pubkey, false),
            AccountMeta::new(obligation_token_output_pubkey, false),
            AccountMeta::new_readonly(obligation_token_owner_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::InitObligationCollateral.pack(),
    }
}

/// Creates a 'RefreshObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn refresh_obligation_collateral(
    program_id: Pubkey,
    obligation_collateral_pubkey: Pubkey,
    deposit_reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    dex_market_pubkey: Pubkey,
    dex_market_order_book_side_pubkey: Pubkey,
    memory_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_collateral_pubkey, false),
            AccountMeta::new_readonly(deposit_reserve_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(dex_market_pubkey, false),
            AccountMeta::new_readonly(dex_market_order_book_side_pubkey, false),
            AccountMeta::new_readonly(memory_pubkey, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::RefreshObligationCollateral.pack(),
    }
}

/// Creates a 'DepositObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn deposit_obligation_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    deposit_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_collateral_pubkey: Pubkey,
    obligation_mint_pubkey: Pubkey,
    obligation_output_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new_readonly(deposit_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_collateral_pubkey, false),
            AccountMeta::new(obligation_mint_pubkey, false),
            AccountMeta::new(obligation_output_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::DepositObligationCollateral { collateral_amount }.pack(),
    }
}

/// Creates a 'WithdrawObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn withdraw_obligation_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    collateral_amount_type: AmountType,
    source_collateral_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_collateral_pubkey: Pubkey,
    obligation_mint_pubkey: Pubkey,
    obligation_input_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new_readonly(withdraw_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_collateral_pubkey, false),
            AccountMeta::new(obligation_mint_pubkey, false),
            AccountMeta::new(obligation_input_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::WithdrawObligationCollateral {
            collateral_amount,
            collateral_amount_type,
        }
        .pack(),
    }
}

/// Creates an 'InitObligationLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn init_obligation_liquidity(
    program_id: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_liquidity_pubkey: Pubkey,
    borrow_reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_liquidity_pubkey, false),
            AccountMeta::new_readonly(borrow_reserve_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::InitObligationLiquidity.pack(),
    }
}

/// Creates a 'RefreshObligationLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn refresh_obligation_liquidity(
    program_id: Pubkey,
    obligation_liquidity_pubkey: Pubkey,
    borrow_reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    dex_market_pubkey: Pubkey,
    dex_market_order_book_side_pubkey: Pubkey,
    memory_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_liquidity_pubkey, false),
            AccountMeta::new_readonly(borrow_reserve_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(dex_market_pubkey, false),
            AccountMeta::new_readonly(dex_market_order_book_side_pubkey, false),
            AccountMeta::new_readonly(memory_pubkey, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::RefreshObligationLiquidity.pack(),
    }
}

/// Creates a 'BorrowObligationLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn borrow_obligation_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    liquidity_amount_type: AmountType,
    source_liquidity_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    borrow_reserve_pubkey: Pubkey,
    borrow_reserve_liquidity_fee_receiver_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_liquidity_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    dex_market_pubkey: Pubkey,
    dex_market_order_book_side_pubkey: Pubkey,
    memory_pubkey: Pubkey,
    host_fee_receiver_pubkey: Option<Pubkey>,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    let mut accounts = vec![
        AccountMeta::new(source_liquidity_pubkey, false),
        AccountMeta::new(destination_liquidity_pubkey, false),
        AccountMeta::new(borrow_reserve_pubkey, false),
        AccountMeta::new(borrow_reserve_liquidity_fee_receiver_pubkey, false),
        AccountMeta::new(obligation_pubkey, false),
        AccountMeta::new(obligation_liquidity_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(dex_market_pubkey, false),
        AccountMeta::new_readonly(dex_market_order_book_side_pubkey, false),
        AccountMeta::new_readonly(memory_pubkey, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    if let Some(host_fee_receiver_pubkey) = host_fee_receiver_pubkey {
        accounts.push(AccountMeta::new(host_fee_receiver_pubkey, false));
    }
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::BorrowObligationLiquidity {
            liquidity_amount,
            liquidity_amount_type,
        }
        .pack(),
    }
}

/// Creates a `RepayObligationLiquidity` instruction
#[allow(clippy::too_many_arguments)]
pub fn repay_obligation_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    liquidity_amount_type: AmountType,
    source_liquidity_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    repay_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_liquidity_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(repay_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_liquidity_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::RepayObligationLiquidity {
            liquidity_amount,
            liquidity_amount_type,
        }
        .pack(),
    }
}

/// Creates a `LiquidateObligation` instruction
#[allow(clippy::too_many_arguments)]
pub fn liquidate_obligation(
    program_id: Pubkey,
    liquidity_amount: u64,
    liquidity_amount_type: AmountType,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    repay_reserve_pubkey: Pubkey,
    repay_reserve_liquidity_supply_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    withdraw_reserve_collateral_supply_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_liquidity_pubkey: Pubkey,
    obligation_collateral_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) =
        Pubkey::find_program_address(&[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]], &program_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(repay_reserve_pubkey, false),
            AccountMeta::new(repay_reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(withdraw_reserve_pubkey, false),
            AccountMeta::new(withdraw_reserve_collateral_supply_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_liquidity_pubkey, false),
            AccountMeta::new(obligation_collateral_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::LiquidateObligation {
            liquidity_amount,
            liquidity_amount_type,
        }
        .pack(),
    }
}
