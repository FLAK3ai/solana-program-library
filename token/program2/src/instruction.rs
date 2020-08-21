//! Instruction types

use crate::{error::TokenError, option::COption};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::mem::size_of;

/// Minimum number of multisignature signers (min N)
pub const MIN_SIGNERS: usize = 1;
/// Maximum number of multisignature signers (max N)
pub const MAX_SIGNERS: usize = 11;

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenInstruction {
    /// Initializes a new mint and optionally deposits all the newly minted tokens in an account.
    ///
    /// The `InitializeMint` instruction requires no signers and MUST be included within
    /// the same Transaction as the system program's `CreateInstruction` that creates the account
    /// being initialized.  Otherwise another party can acquire ownership of the uninitialized account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The mint to initialize.
    ///   1. If supply is non-zero: `[writable]` The account to hold all the newly minted tokens.
    ///
    InitializeMint {
        /// Initial amount of tokens to mint.
        amount: u64,
        /// Number of base 10 digits to the right of the decimal place.
        decimals: u8,
        /// The owner/multisignature of the mint if supply is non-zero. If present, further minting
        /// is supported.
        owner: COption<Pubkey>,
        /// The freeze authority/multisignature of the mint.
        freeze_authority: COption<Pubkey>,
    },
    /// Initializes a new account to hold tokens.  If this account is associated with the native mint
    /// then the token balance of the initialized account will be equal to the amount of SOL in the account.
    ///
    /// The `InitializeAccount` instruction requires no signers and MUST be included within
    /// the same Transaction as the system program's `CreateInstruction` that creates the account
    /// being initialized.  Otherwise another party can acquire ownership of the uninitialized account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]`  The account to initialize.
    ///   1. `[]` The mint this account will be associated with.
    ///   2. `[]` The new account's owner/multisignature.
    InitializeAccount,
    /// Initializes a multisignature account with N provided signers.
    ///
    /// Multisignature accounts can used in place of any single owner/delegate accounts in any
    /// token instruction that require an owner/delegate to be present.  The variant field represents the
    /// number of signers (M) required to validate this multisignature account.
    ///
    /// The `InitializeMultisig` instruction requires no signers and MUST be included within
    /// the same Transaction as the system program's `CreateInstruction` that creates the account
    /// being initialized.  Otherwise another party can acquire ownership of the uninitialized account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The multisignature account to initialize.
    ///   1. ..1+N. `[]` The signer accounts, must equal to N where 1 <= N <= 11.
    InitializeMultisig {
        /// The number of signers (M) required to validate this multisignature account.
        m: u8,
    },
    /// Transfers tokens from one account to another either directly or via a delegate.  If this
    /// account is associated with the native mint then equal amounts of SOL and Tokens will be
    /// transferred to the destination account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[writable]` The destination account.
    ///   2. '[signer]' The source account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[writable]` The destination account.
    ///   2. '[]' The source account's multisignature owner/delegate.
    ///   3. ..3+M '[signer]' M signer accounts.
    Transfer {
        /// The amount of tokens to transfer.
        amount: u64,
    },
    /// Approves a delegate.  A delegate is given the authority over
    /// tokens on behalf of the source account's owner.

    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The delegate.
    ///   2. `[signer]` The source account owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The delegate.
    ///   2. '[]' The source account's multisignature owner.
    ///   3. ..3+M '[signer]' M signer accounts
    Approve {
        /// The amount of tokens the delegate is approved for.
        amount: u64,
    },
    /// Revokes the delegate's authority.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The source account.
    ///   1. `[signer]` The source account owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The source account.
    ///   1. '[]' The source account's multisignature owner.
    ///   2. ..2+M '[signer]' M signer accounts
    Revoke,
    /// Sets a new owner of a mint or account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The mint or account to change the owner of.
    ///   1. `[]` The new owner/delegate/multisignature.
    ///   2. `[signer]` The owner of the mint or account.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The mint or account to change the owner of.
    ///   1. `[]` The new owner/delegate/multisignature.
    ///   2. `[]` The mint's or account's multisignature owner.
    ///   3. ..3+M '[signer]' M signer accounts
    SetAuthority {
        /// The type of authority to update.
        authority_type: AuthorityType,
    },
    /// Mints new tokens to an account.  The native mint does not support minting.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[signer]` The mint's owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[]` The mint's multisignature owner.
    ///   3. ..3+M '[signer]' M signer accounts.
    MintTo {
        /// The amount of new tokens to mint.
        amount: u64,
    },
    /// Burns tokens by removing them from an account.  `Burn` does not support accounts
    /// associated with the native mint, use `CloseAccount` instead.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The account to burn from.
    ///   1. `[signer]` The account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The account to burn from.
    ///   1. `[]` The account's multisignature owner/delegate.
    ///   2. ..2+M '[signer]' M signer accounts.
    Burn {
        /// The amount of tokens to burn.
        amount: u64,
    },
    /// Close an account by transferring all its SOL to the destination account.
    /// Non-native accounts may only be closed if its token amount is zero.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The account to close.
    ///   1. '[writable]' The destination account.
    ///   2. `[signer]` The account's owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The account to close.
    ///   1. '[writable]' The destination account.
    ///   2. `[]` The account's multisignature owner.
    ///   3. ..3+M '[signer]' M signer accounts.
    CloseAccount,
    /// Freeze an Initialized account or unfreeze a Frozen account, using the Mint's
    /// freeze_authority (if set).
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The account to freeze.
    ///   1. '[]' The token mint.
    ///   2. `[signer]` The mint freeze authority.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The account to freeze.
    ///   1. '[]' The token mint.
    ///   2. `[]` The mint's multisignature freeze authority.
    ///   3. ..3+M '[signer]' M signer accounts.
    FreezeAccount {
        /// Explicitly: whether to freeze the account if it is Initialized. `false` means to
        /// unfreeze if the account is Frozen.
        freeze: bool,
    },
}
impl TokenInstruction {
    /// Unpacks a byte buffer into a [TokenInstruction](enum.TokenInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < size_of::<u8>() {
            return Err(TokenError::InvalidInstruction.into());
        }
        Ok(match input[0] {
            0 => {
                if input.len()
                    < size_of::<u8>() + size_of::<u64>() + size_of::<u8>() + size_of::<bool>()
                {
                    return Err(TokenError::InvalidInstruction.into());
                }
                let mut input_len = 0;
                input_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let amount = unsafe { *(&input[input_len] as *const u8 as *const u64) };
                input_len += size_of::<u64>();
                let decimals = unsafe { *(&input[input_len] as *const u8) };
                input_len += size_of::<u8>();

                let owner = match input[input_len] {
                    0 => {
                        input_len += size_of::<u8>();
                        COption::None
                    }
                    1 => {
                        input_len += size_of::<u8>();
                        #[allow(clippy::cast_ptr_alignment)]
                        let owner = unsafe { *(&input[input_len] as *const u8 as *const Pubkey) };
                        input_len += size_of::<Pubkey>();
                        COption::Some(owner)
                    }
                    _ => {
                        return Err(TokenError::InvalidInstruction.into());
                    }
                };

                let freeze_authority = match input[input_len] {
                    0 => COption::None,
                    1 => {
                        input_len += size_of::<u8>();
                        #[allow(clippy::cast_ptr_alignment)]
                        let freeze_authority =
                            unsafe { *(&input[input_len] as *const u8 as *const Pubkey) };
                        COption::Some(freeze_authority)
                    }
                    _ => {
                        return Err(TokenError::InvalidInstruction.into());
                    }
                };

                Self::InitializeMint {
                    owner,
                    freeze_authority,
                    amount,
                    decimals,
                }
            }
            1 => Self::InitializeAccount,
            2 => {
                if input.len() < size_of::<u8>() + size_of::<u8>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                #[allow(clippy::cast_ptr_alignment)]
                let m = unsafe { *(&input[1] as *const u8) };
                Self::InitializeMultisig { m }
            }
            3 => {
                if input.len() < size_of::<u8>() + size_of::<u64>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                #[allow(clippy::cast_ptr_alignment)]
                let amount = unsafe { *(&input[size_of::<u8>()] as *const u8 as *const u64) };
                Self::Transfer { amount }
            }
            4 => {
                if input.len() < size_of::<u8>() + size_of::<u64>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                #[allow(clippy::cast_ptr_alignment)]
                let amount = unsafe { *(&input[size_of::<u8>()] as *const u8 as *const u64) };
                Self::Approve { amount }
            }
            5 => Self::Revoke,
            6 => {
                if input.len() < size_of::<u8>() + size_of::<u8>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                let authority_type = match input[1] {
                    0 => AuthorityType::Owner,
                    1 => AuthorityType::Freezer,
                    _ => return Err(TokenError::InvalidInstruction.into()),
                };
                Self::SetAuthority { authority_type }
            }
            7 => {
                if input.len() < size_of::<u8>() + size_of::<u64>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                #[allow(clippy::cast_ptr_alignment)]
                let amount = unsafe { *(&input[size_of::<u8>()] as *const u8 as *const u64) };
                Self::MintTo { amount }
            }
            8 => {
                if input.len() < size_of::<u8>() + size_of::<u64>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                #[allow(clippy::cast_ptr_alignment)]
                let amount = unsafe { *(&input[size_of::<u8>()] as *const u8 as *const u64) };
                Self::Burn { amount }
            }
            9 => Self::CloseAccount,
            10 => {
                if input.len() < size_of::<u8>() + size_of::<u8>() {
                    return Err(TokenError::InvalidInstruction.into());
                }
                #[allow(clippy::cast_ptr_alignment)]
                let freeze = unsafe { *(&input[size_of::<u8>()] as *const u8 as *const bool) };
                Self::FreezeAccount { freeze }
            }
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }

    /// Packs a [TokenInstruction](enum.TokenInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut output = vec![0u8; size_of::<TokenInstruction>()];
        let mut output_len = 0;
        match self {
            Self::InitializeMint {
                owner,
                freeze_authority,
                amount,
                decimals,
            } => {
                output[output_len] = 0;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut u64) };
                *value = *amount;
                output_len += size_of::<u64>();

                let value = unsafe { &mut *(&mut output[output_len] as *mut u8) };
                *value = *decimals;
                output_len += size_of::<u8>();

                match owner {
                    COption::Some(owner) => {
                        output[output_len] = 1;
                        output_len += size_of::<u8>();

                        #[allow(clippy::cast_ptr_alignment)]
                        let value =
                            unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut Pubkey) };
                        *value = *owner;
                        output_len += size_of::<Pubkey>();
                    }
                    COption::None => {
                        output[output_len] = 0;
                        output_len += size_of::<u8>();
                    }
                }

                match freeze_authority {
                    COption::Some(freeze_authority) => {
                        output[output_len] = 1;
                        output_len += size_of::<u8>();

                        #[allow(clippy::cast_ptr_alignment)]
                        let value =
                            unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut Pubkey) };
                        *value = *freeze_authority;
                        output_len += size_of::<Pubkey>();
                    }
                    COption::None => {
                        output[output_len] = 0;
                        output_len += size_of::<u8>();
                    }
                }
            }
            Self::InitializeAccount => {
                output[output_len] = 1;
                output_len += size_of::<u8>();
            }
            Self::InitializeMultisig { m } => {
                output[output_len] = 2;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut u8) };
                *value = *m;
                output_len += size_of::<u8>();
            }
            Self::Transfer { amount } => {
                output[output_len] = 3;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut u64) };
                *value = *amount;
                output_len += size_of::<u64>();
            }
            Self::Approve { amount } => {
                output[output_len] = 4;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut u64) };
                *value = *amount;
                output_len += size_of::<u64>();
            }
            Self::Revoke => {
                output[output_len] = 5;
                output_len += size_of::<u8>();
            }
            Self::SetAuthority { authority_type } => {
                output[output_len] = 6;
                output_len += size_of::<u8>();

                let byte = match authority_type {
                    AuthorityType::Owner => 0,
                    AuthorityType::Freezer => 1,
                };
                output[output_len] = byte;
                output_len += size_of::<u8>();
            }
            Self::MintTo { amount } => {
                output[output_len] = 7;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut u64) };
                *value = *amount;
                output_len += size_of::<u64>();
            }
            Self::Burn { amount } => {
                output[output_len] = 8;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut u64) };
                *value = *amount;
                output_len += size_of::<u64>();
            }
            Self::CloseAccount => {
                output[output_len] = 9;
                output_len += size_of::<u8>();
            }
            Self::FreezeAccount { freeze } => {
                output[output_len] = 10;
                output_len += size_of::<u8>();

                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[output_len] as *mut u8 as *mut bool) };
                *value = *freeze;
                output_len += size_of::<u8>();
            }
        }

        output.truncate(output_len);
        Ok(output)
    }
}

/// Specifies the authority type for SetAuthority instructions
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorityType {
    /// General authority, valid for Account and Mint
    Owner,
    /// Freeze authority, only valid for Mint
    Freezer,
}

/// Creates a 'InitializeMint' instruction.
pub fn initialize_mint(
    token_program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    account_pubkey: Option<&Pubkey>,
    owner_pubkey: Option<&Pubkey>,
    freeze_pubkey: Option<&Pubkey>,
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let owner = if let Some(owner) = owner_pubkey {
        COption::Some(*owner)
    } else {
        COption::None
    };
    let freeze_authority = if let Some(freeze_authority) = freeze_pubkey {
        COption::Some(*freeze_authority)
    } else {
        COption::None
    };
    let data = TokenInstruction::InitializeMint {
        owner,
        freeze_authority,
        amount,
        decimals,
    }
    .pack()?;

    let mut accounts = vec![AccountMeta::new(*mint_pubkey, false)];
    if amount != 0 {
        match account_pubkey {
            Some(pubkey) => accounts.push(AccountMeta::new(*pubkey, false)),
            None => {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
        }
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `InitializeAccount` instruction.
pub fn initialize_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::InitializeAccount.pack()?;

    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, false),
    ];

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `InitializeMultisig` instruction.
pub fn initialize_multisig(
    token_program_id: &Pubkey,
    multisig_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    m: u8,
) -> Result<Instruction, ProgramError> {
    if !is_valid_signer_index(m as usize)
        || !is_valid_signer_index(signer_pubkeys.len())
        || m as usize > signer_pubkeys.len()
    {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let data = TokenInstruction::InitializeMultisig { m }.pack()?;

    let mut accounts = Vec::with_capacity(1 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*multisig_pubkey, false));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, false));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `Transfer` instruction.
pub fn transfer(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Transfer { amount }.pack()?;

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new(*destination_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *authority_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates an `Approve` instruction.
pub fn approve(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    delegate_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Approve { amount }.pack()?;

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*delegate_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `Revoke` instruction.
pub fn revoke(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Revoke.pack()?;

    let mut accounts = Vec::with_capacity(2 + signer_pubkeys.len());
    accounts.push(AccountMeta::new_readonly(*source_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `SetAuthority` instruction.
pub fn set_authority(
    token_program_id: &Pubkey,
    owned_pubkey: &Pubkey,
    new_owner_pubkey: &Pubkey,
    authority_type: AuthorityType,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::SetAuthority { authority_type }.pack()?;

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*owned_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*new_owner_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `MintTo` instruction.
pub fn mint_to(
    token_program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::MintTo { amount }.pack()?;

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*mint_pubkey, false));
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `Burn` instruction.
pub fn burn(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Burn { amount }.pack()?;

    let mut accounts = Vec::with_capacity(2 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *authority_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `CloseAccount` instruction.
pub fn close_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::CloseAccount.pack()?;

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new(*destination_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `FreezeAccount` instruction.
pub fn freeze_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    freeze: bool,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::FreezeAccount { freeze }.pack()?;

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Utility function that checks index is between MIN_SIGNERS and MAX_SIGNERS
pub fn is_valid_signer_index(index: usize) -> bool {
    !(index < MIN_SIGNERS || index > MAX_SIGNERS)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_packing() {
        let check = TokenInstruction::InitializeMint {
            amount: 1,
            decimals: 2,
            owner: COption::None,
            freeze_authority: COption::None,
        };
        let packed = check.pack().unwrap();
        let expect = Vec::from([0u8, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::InitializeMint {
            amount: 1,
            decimals: 2,
            owner: COption::Some(Pubkey::new(&[2u8; 32])),
            freeze_authority: COption::Some(Pubkey::new(&[3u8; 32])),
        };
        let packed = check.pack().unwrap();
        let expect = vec![
            0u8, 1, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        ];
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::InitializeAccount;
        let packed = check.pack().unwrap();
        let expect = Vec::from([1u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::InitializeMultisig { m: 1 };
        let packed = check.pack().unwrap();
        let expect = Vec::from([2u8, 1]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::Transfer { amount: 1 };
        let packed = check.pack().unwrap();
        let expect = Vec::from([3u8, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::Approve { amount: 1 };
        let packed = check.pack().unwrap();
        let expect = Vec::from([4u8, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::Revoke;
        let packed = check.pack().unwrap();
        let expect = Vec::from([5u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::SetAuthority {
            authority_type: AuthorityType::Freezer,
        };
        let packed = check.pack().unwrap();
        let expect = Vec::from([6u8, 1]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::MintTo { amount: 1 };
        let packed = check.pack().unwrap();
        let expect = Vec::from([7u8, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::Burn { amount: 1 };
        let packed = check.pack().unwrap();
        let expect = Vec::from([8u8, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::CloseAccount;
        let packed = check.pack().unwrap();
        let expect = Vec::from([9u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
    }
}
