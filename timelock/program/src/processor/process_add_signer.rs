//! Program state processor
use crate::{
    state::timelock_program::TimelockProgram,
    state::timelock_set::TimelockSet,
    utils::{
        assert_draft, assert_initialized, assert_is_permissioned, assert_proper_signatory_mint,
        assert_same_version_as_program, assert_token_program_is_correct, spl_token_init_account,
        spl_token_mint_to, TokenInitializeAccountParams, TokenMintToParams,
    },
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

/// Adds a signer
pub fn process_add_signer(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let new_signatory_account_info = next_account_info(account_info_iter)?;
    let signatory_mint_info = next_account_info(account_info_iter)?;
    let admin_account_info = next_account_info(account_info_iter)?;
    let admin_validation_account_info = next_account_info(account_info_iter)?;
    let timelock_set_account_info = next_account_info(account_info_iter)?;
    let timelock_program_account_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let token_program_account_info = next_account_info(account_info_iter)?;

    let timelock_set: TimelockSet = assert_initialized(timelock_set_account_info)?;
    let timelock_program: TimelockProgram = assert_initialized(timelock_program_account_info)?;
    assert_same_version_as_program(&timelock_program, &timelock_set)?;
    assert_token_program_is_correct(&timelock_program, token_program_account_info)?;
    assert_proper_signatory_mint(&timelock_set, signatory_mint_info)?;
    assert_draft(&timelock_set)?;
    assert_is_permissioned(
        admin_account_info,
        admin_validation_account_info,
        timelock_program_account_info,
        token_program_account_info,
    )?;

    let (_, bump_seed) =
        Pubkey::find_program_address(&[timelock_set_account_info.key.as_ref()], program_id);

    let authority_signer_seeds = &[token_program_account_info.key.as_ref(), &[bump_seed]];

    // Give this person a token!
    spl_token_init_account(TokenInitializeAccountParams {
        account: new_signatory_account_info.clone(),
        mint: signatory_mint_info.clone(),
        owner: timelock_program_account_info.clone(),
        rent: rent_info.clone(),
        token_program: token_program_account_info.clone(),
    })?;

    spl_token_mint_to(TokenMintToParams {
        mint: signatory_mint_info.clone(),
        destination: new_signatory_account_info.clone(),
        amount: 1,
        authority: timelock_program_account_info.clone(),
        authority_signer_seeds: authority_signer_seeds,
        token_program: token_program_account_info.clone(),
    })?;
    Ok(())
}
