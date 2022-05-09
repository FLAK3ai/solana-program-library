//! Fund info account management.

use {
    solana_farm_sdk::{
        error::FarmError,
        program::clock,
        refdb,
        refdb::{RefDB, Reference, ReferenceType, StorageType},
        string::{str_to_as64, ArrayString64},
    },
    solana_program::{
        account_info::AccountInfo, clock::UnixTimestamp, entrypoint::ProgramResult,
        program_error::ProgramError, pubkey::Pubkey,
    },
    std::cell::RefMut,
};

pub struct FundInfo<'a, 'b> {
    pub key: &'a Pubkey,
    pub data: RefMut<'a, &'b mut [u8]>,
}

impl<'a, 'b> FundInfo<'a, 'b> {
    pub const LEN: usize = StorageType::get_storage_size_for_records(ReferenceType::U64, 22);
    pub const DEPOSIT_START_TIME_INDEX: usize = 0;
    pub const DEPOSIT_END_TIME_INDEX: usize = 1;
    pub const DEPOSIT_APPROVAL_REQUIRED_INDEX: usize = 2;
    pub const DEPOSIT_LIMIT_USD_INDEX: usize = 3;
    pub const DEPOSIT_FEE_INDEX: usize = 4;
    pub const WITHDRAWAL_START_TIME_INDEX: usize = 5;
    pub const WITHDRAWAL_END_TIME_INDEX: usize = 6;
    pub const WITHDRAWAL_APPROVAL_REQUIRED_INDEX: usize = 7;
    pub const WITHDRAWAL_LIMIT_INDEX: usize = 8;
    pub const WITHDRAWAL_FEE_INDEX: usize = 9;
    pub const ASSETS_LIMIT_USD_INDEX: usize = 10;
    pub const ASSETS_MAX_UPDATE_AGE_SEC_INDEX: usize = 11;
    pub const ASSETS_MAX_PRICE_ERROR_INDEX: usize = 12;
    pub const ASSETS_MAX_PRICE_AGE_SEC_INDEX: usize = 13;
    pub const AMOUNT_INVESTED_USD_INDEX: usize = 14;
    pub const AMOUNT_REMOVED_USD_INDEX: usize = 15;
    pub const CURRENT_ASSETS_USD_INDEX: usize = 16;
    pub const ASSETS_UPDATE_TIME_INDEX: usize = 17;
    pub const ADMIN_ACTION_TIME_INDEX: usize = 18;
    pub const LIQUIDATION_START_TIME_INDEX: usize = 19;
    pub const LIQUIDATION_AMOUNT_USD_INDEX: usize = 20;
    pub const LIQUIDATION_AMOUNT_TOKENS_INDEX: usize = 21;

    pub fn new(account: &'a AccountInfo<'b>) -> Self {
        Self {
            key: account.key,
            data: account.data.borrow_mut(),
        }
    }

    pub fn init(&mut self, refdb_name: &ArrayString64) -> ProgramResult {
        if RefDB::is_initialized(&self.data) {
            return Ok(());
        }
        RefDB::init(&mut self.data, refdb_name, ReferenceType::U64)?;

        self.init_refdb_field(
            FundInfo::DEPOSIT_START_TIME_INDEX,
            "DepositStartTime",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::DEPOSIT_END_TIME_INDEX,
            "DepositEndTime",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::DEPOSIT_APPROVAL_REQUIRED_INDEX,
            "DepositApprovalRequired",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::DEPOSIT_LIMIT_USD_INDEX,
            "DepositLimitUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::DEPOSIT_FEE_INDEX,
            "DepositFee",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::WITHDRAWAL_START_TIME_INDEX,
            "WithdrawalStartTime",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::WITHDRAWAL_END_TIME_INDEX,
            "WithdrawalEndTime",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::WITHDRAWAL_APPROVAL_REQUIRED_INDEX,
            "WithdrawalApprovalRequired",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::WITHDRAWAL_LIMIT_INDEX,
            "WithdrawalLimitUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::WITHDRAWAL_FEE_INDEX,
            "WithdrawalFee",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::ASSETS_LIMIT_USD_INDEX,
            "AssetsLimitUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::ASSETS_MAX_UPDATE_AGE_SEC_INDEX,
            "AssetsMaxUpdateAgeSec",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::ASSETS_MAX_PRICE_ERROR_INDEX,
            "AssetsMaxPriceError",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::ASSETS_MAX_PRICE_AGE_SEC_INDEX,
            "AssetsMaxPriceAgeSec",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::AMOUNT_INVESTED_USD_INDEX,
            "AmountIvestedUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::AMOUNT_REMOVED_USD_INDEX,
            "AmountRemovedUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::CURRENT_ASSETS_USD_INDEX,
            "CurrentAssetsUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::ASSETS_UPDATE_TIME_INDEX,
            "AssetsUpdateTime",
            Reference::U64 {
                data: clock::get_time_as_u64()?,
            },
        )?;
        self.init_refdb_field(
            FundInfo::ADMIN_ACTION_TIME_INDEX,
            "AdminActionTime",
            Reference::U64 {
                data: clock::get_time_as_u64()?,
            },
        )?;
        self.init_refdb_field(
            FundInfo::LIQUIDATION_START_TIME_INDEX,
            "LiquidationStartTime",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::LIQUIDATION_AMOUNT_USD_INDEX,
            "LiquidationAmountUsd",
            Reference::U64 { data: 0 },
        )?;
        self.init_refdb_field(
            FundInfo::LIQUIDATION_AMOUNT_TOKENS_INDEX,
            "LiquidationAmountTokens",
            Reference::U64 { data: 0 },
        )
    }

    pub fn set_deposit_start_time(&mut self, deposit_start_time: UnixTimestamp) -> ProgramResult {
        if deposit_start_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::DEPOSIT_START_TIME_INDEX,
            &Reference::U64 {
                data: deposit_start_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_deposit_end_time(&mut self, deposit_end_time: UnixTimestamp) -> ProgramResult {
        if deposit_end_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::DEPOSIT_END_TIME_INDEX,
            &Reference::U64 {
                data: deposit_end_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_deposit_approval_required(
        &mut self,
        deposit_approval_required: bool,
    ) -> ProgramResult {
        RefDB::update_at(
            &mut self.data,
            FundInfo::DEPOSIT_APPROVAL_REQUIRED_INDEX,
            &Reference::U64 {
                data: deposit_approval_required as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_deposit_limit_usd(&mut self, deposit_limit_usd: f64) -> ProgramResult {
        if deposit_limit_usd < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::DEPOSIT_LIMIT_USD_INDEX,
            &Reference::U64 {
                data: deposit_limit_usd.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_deposit_fee(&mut self, deposit_fee: f64) -> ProgramResult {
        if !(0.0..=1.0).contains(&deposit_fee) {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::DEPOSIT_FEE_INDEX,
            &Reference::U64 {
                data: deposit_fee.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_withdrawal_start_time(
        &mut self,
        withdrawal_start_time: UnixTimestamp,
    ) -> ProgramResult {
        if withdrawal_start_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::WITHDRAWAL_START_TIME_INDEX,
            &Reference::U64 {
                data: withdrawal_start_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_withdrawal_end_time(&mut self, withdrawal_end_time: UnixTimestamp) -> ProgramResult {
        if withdrawal_end_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::WITHDRAWAL_END_TIME_INDEX,
            &Reference::U64 {
                data: withdrawal_end_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_withdrawal_approval_required(
        &mut self,
        withdrawal_approval_required: bool,
    ) -> ProgramResult {
        RefDB::update_at(
            &mut self.data,
            FundInfo::WITHDRAWAL_APPROVAL_REQUIRED_INDEX,
            &Reference::U64 {
                data: withdrawal_approval_required as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_withdrawal_limit_usd(&mut self, withdrawal_limit: f64) -> ProgramResult {
        if withdrawal_limit < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::WITHDRAWAL_LIMIT_INDEX,
            &Reference::U64 {
                data: withdrawal_limit.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_withdrawal_fee(&mut self, withdrawal_fee: f64) -> ProgramResult {
        if !(0.0..=1.0).contains(&withdrawal_fee) {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::WITHDRAWAL_FEE_INDEX,
            &Reference::U64 {
                data: withdrawal_fee.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_assets_limit_usd(&mut self, assets_limit_usd: f64) -> ProgramResult {
        if assets_limit_usd < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::ASSETS_LIMIT_USD_INDEX,
            &Reference::U64 {
                data: assets_limit_usd.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_assets_max_update_age_sec(
        &mut self,
        assets_max_update_age_sec: u64,
    ) -> ProgramResult {
        RefDB::update_at(
            &mut self.data,
            FundInfo::ASSETS_MAX_UPDATE_AGE_SEC_INDEX,
            &Reference::U64 {
                data: assets_max_update_age_sec,
            },
        )
        .map(|_| ())
    }

    pub fn set_assets_max_price_error(&mut self, assets_max_price_error: f64) -> ProgramResult {
        if assets_max_price_error < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::ASSETS_MAX_PRICE_ERROR_INDEX,
            &Reference::U64 {
                data: assets_max_price_error.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_assets_max_price_age_sec(&mut self, assets_max_price_age_sec: u64) -> ProgramResult {
        RefDB::update_at(
            &mut self.data,
            FundInfo::ASSETS_MAX_PRICE_AGE_SEC_INDEX,
            &Reference::U64 {
                data: assets_max_price_age_sec,
            },
        )
        .map(|_| ())
    }

    pub fn set_amount_invested_usd(&mut self, amount_invested_usd: f64) -> ProgramResult {
        if amount_invested_usd < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::AMOUNT_INVESTED_USD_INDEX,
            &Reference::U64 {
                data: amount_invested_usd.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_amount_removed_usd(&mut self, amount_removed_usd: f64) -> ProgramResult {
        if amount_removed_usd < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::AMOUNT_REMOVED_USD_INDEX,
            &Reference::U64 {
                data: amount_removed_usd.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_current_assets_usd(&mut self, current_assets_usd: f64) -> ProgramResult {
        if current_assets_usd < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::CURRENT_ASSETS_USD_INDEX,
            &Reference::U64 {
                data: current_assets_usd.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_assets_update_time(&mut self, assets_update_time: UnixTimestamp) -> ProgramResult {
        if assets_update_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::ASSETS_UPDATE_TIME_INDEX,
            &Reference::U64 {
                data: assets_update_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_admin_action_time(&mut self, admin_action_time: UnixTimestamp) -> ProgramResult {
        if admin_action_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::ADMIN_ACTION_TIME_INDEX,
            &Reference::U64 {
                data: admin_action_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn update_admin_action_time(&mut self) -> ProgramResult {
        self.set_admin_action_time(clock::get_time()?)
    }

    pub fn set_liquidation_start_time(
        &mut self,
        liquidation_start_time: UnixTimestamp,
    ) -> ProgramResult {
        if liquidation_start_time < 0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::LIQUIDATION_START_TIME_INDEX,
            &Reference::U64 {
                data: liquidation_start_time as u64,
            },
        )
        .map(|_| ())
    }

    pub fn set_liquidation_amount_usd(&mut self, liquidation_amount_usd: f64) -> ProgramResult {
        if liquidation_amount_usd < 0.0 {
            return Err(FarmError::InvalidValue.into());
        }
        RefDB::update_at(
            &mut self.data,
            FundInfo::LIQUIDATION_AMOUNT_USD_INDEX,
            &Reference::U64 {
                data: liquidation_amount_usd.to_bits(),
            },
        )
        .map(|_| ())
    }

    pub fn set_liquidation_amount_tokens(
        &mut self,
        liquidation_amount_tokens: u64,
    ) -> ProgramResult {
        RefDB::update_at(
            &mut self.data,
            FundInfo::LIQUIDATION_AMOUNT_TOKENS_INDEX,
            &Reference::U64 {
                data: liquidation_amount_tokens,
            },
        )
        .map(|_| ())
    }

    pub fn is_deposit_allowed(&self) -> Result<bool, ProgramError> {
        if self.get_liquidation_start_time()? > 0 {
            return Ok(false);
        }
        let deposit_start_time =
            if let Some(rec) = RefDB::read_at(&self.data, FundInfo::DEPOSIT_START_TIME_INDEX)? {
                if let Reference::U64 { data } = rec.reference {
                    data as UnixTimestamp
                } else {
                    return Err(FarmError::InvalidRefdbRecord.into());
                }
            } else {
                return Err(FarmError::InvalidRefdbRecord.into());
            };
        let deposit_end_time =
            if let Some(rec) = RefDB::read_at(&self.data, FundInfo::DEPOSIT_END_TIME_INDEX)? {
                if let Reference::U64 { data } = rec.reference {
                    data as UnixTimestamp
                } else {
                    return Err(FarmError::InvalidRefdbRecord.into());
                }
            } else {
                return Err(FarmError::InvalidRefdbRecord.into());
            };
        let current_time = clock::get_time()?;
        Ok(current_time > 0
            && current_time >= deposit_start_time
            && current_time < deposit_end_time)
    }

    pub fn is_deposit_approval_required(&self) -> Result<bool, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::DEPOSIT_APPROVAL_REQUIRED_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data > 0);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_deposit_limit_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::DEPOSIT_LIMIT_USD_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_deposit_fee(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::DEPOSIT_FEE_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn is_withdrawal_allowed(&self) -> Result<bool, ProgramError> {
        if self.get_liquidation_start_time()? > 0 {
            return Ok(false);
        }
        let withdrawal_start_time =
            if let Some(rec) = RefDB::read_at(&self.data, FundInfo::WITHDRAWAL_START_TIME_INDEX)? {
                if let Reference::U64 { data } = rec.reference {
                    data as UnixTimestamp
                } else {
                    return Err(FarmError::InvalidRefdbRecord.into());
                }
            } else {
                return Err(FarmError::InvalidRefdbRecord.into());
            };
        let withdrawal_end_time =
            if let Some(rec) = RefDB::read_at(&self.data, FundInfo::WITHDRAWAL_END_TIME_INDEX)? {
                if let Reference::U64 { data } = rec.reference {
                    data as UnixTimestamp
                } else {
                    return Err(FarmError::InvalidRefdbRecord.into());
                }
            } else {
                return Err(FarmError::InvalidRefdbRecord.into());
            };
        let current_time = clock::get_time()?;
        Ok(current_time > 0
            && current_time >= withdrawal_start_time
            && current_time < withdrawal_end_time)
    }

    pub fn is_withdrawal_approval_required(&self) -> Result<bool, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::WITHDRAWAL_APPROVAL_REQUIRED_INDEX)?
        {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data > 0);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_withdrawal_limit_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::WITHDRAWAL_LIMIT_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_withdrawal_fee(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::WITHDRAWAL_FEE_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_assets_limit_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::ASSETS_LIMIT_USD_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_assets_max_update_age_sec(&self) -> Result<u64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::ASSETS_MAX_UPDATE_AGE_SEC_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_assets_max_price_error(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::ASSETS_MAX_PRICE_ERROR_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_assets_max_price_age_sec(&self) -> Result<u64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::ASSETS_MAX_PRICE_AGE_SEC_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_amount_invested_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::AMOUNT_INVESTED_USD_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_amount_removed_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::AMOUNT_REMOVED_USD_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_current_assets_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::CURRENT_ASSETS_USD_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_assets_update_time(&self) -> Result<UnixTimestamp, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::ASSETS_UPDATE_TIME_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data as UnixTimestamp);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_admin_action_time(&self) -> Result<UnixTimestamp, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::ADMIN_ACTION_TIME_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data as UnixTimestamp);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_liquidation_start_time(&self) -> Result<UnixTimestamp, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::LIQUIDATION_START_TIME_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data as UnixTimestamp);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_liquidation_amount_usd(&self) -> Result<f64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::LIQUIDATION_AMOUNT_USD_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(f64::from_bits(data));
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    pub fn get_liquidation_amount_tokens(&self) -> Result<u64, ProgramError> {
        if let Some(rec) = RefDB::read_at(&self.data, FundInfo::LIQUIDATION_AMOUNT_TOKENS_INDEX)? {
            if let Reference::U64 { data } = rec.reference {
                return Ok(data);
            }
        }
        Err(FarmError::InvalidRefdbRecord.into())
    }

    // private helpers
    fn init_refdb_field(
        &mut self,
        index: usize,
        field_name: &str,
        reference: Reference,
    ) -> ProgramResult {
        RefDB::write(
            &mut self.data,
            &refdb::Record {
                index: Some(index as u32),
                counter: 0,
                tag: 0,
                name: str_to_as64(field_name)?,
                reference,
            },
        )
        .map(|_| ())
    }
}
