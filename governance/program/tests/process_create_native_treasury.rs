#![cfg(feature = "test-sbf")]

use solana_program_test::*;

mod program_test;

use program_test::*;
use solana_program::pubkey::Pubkey;
use spl_governance::state::proposal_extra_signer::get_proposal_extra_account_address;

#[tokio::test]
async fn test_create_native_treasury() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let governance_cookie = governance_test
        .with_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    // Act
    let native_treasury_cookie = governance_test
        .with_native_treasury(&governance_cookie)
        .await;

    // Assert

    let native_treasury_account = governance_test
        .get_native_treasury_account(&native_treasury_cookie.address)
        .await;

    assert_eq!(native_treasury_cookie.account, native_treasury_account);
}

#[tokio::test]
async fn test_execute_transfer_from_native_treasury() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = governance_test
        .with_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    governance_test
        .with_native_treasury(&governance_cookie)
        .await;

    let mut proposal_cookie = governance_test
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let signatory_record_cookie = governance_test
        .with_signatory(&proposal_cookie, &token_owner_record_cookie)
        .await
        .unwrap();

    let wallet_cookie = governance_test.bench.with_wallet().await;
    let transfer_amount = 100;

    let proposal_transaction_cookie = governance_test
        .with_native_transfer_transaction(
            &governance_cookie,
            &mut proposal_cookie,
            &token_owner_record_cookie,
            &wallet_cookie,
            transfer_amount,
        )
        .await
        .unwrap();

    governance_test
        .sign_off_proposal(&proposal_cookie, &signatory_record_cookie)
        .await
        .unwrap();

    governance_test
        .with_cast_yes_no_vote(&proposal_cookie, &token_owner_record_cookie, YesNoVote::Yes)
        .await
        .unwrap();

    // Advance timestamp past hold_up_time
    governance_test
        .advance_clock_by_min_timespan(proposal_transaction_cookie.account.hold_up_time as u64)
        .await;

    // Act
    governance_test
        .execute_proposal_transaction(&proposal_cookie, &proposal_transaction_cookie)
        .await
        .unwrap();

    // Assert
    let wallet_account = governance_test
        .bench
        .get_account(&wallet_cookie.address)
        .await
        .unwrap();

    assert_eq!(
        wallet_account.lamports,
        wallet_cookie.account.lamports + transfer_amount
    )
}

#[tokio::test]
async fn test_create_account_from_native_treasury() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = governance_test
        .with_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    governance_test
        .with_native_treasury(&governance_cookie)
        .await;

    let mut proposal_cookie = governance_test
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let signatory_record_cookie = governance_test
        .with_signatory(&proposal_cookie, &token_owner_record_cookie)
        .await
        .unwrap();

    let expected_owner = Pubkey::new_unique();

    let proposal_transaction_cookie = governance_test
        .with_create_account_transaction(
            &governance_cookie,
            &mut proposal_cookie,
            &token_owner_record_cookie,
            &expected_owner,
            1,
        )
        .await
        .unwrap();

    governance_test
        .sign_off_proposal(&proposal_cookie, &signatory_record_cookie)
        .await
        .unwrap();

    governance_test
        .with_cast_yes_no_vote(&proposal_cookie, &token_owner_record_cookie, YesNoVote::Yes)
        .await
        .unwrap();

    // Advance timestamp past hold_up_time
    governance_test
        .advance_clock_by_min_timespan(proposal_transaction_cookie.account.hold_up_time as u64)
        .await;

    // Act
    governance_test
        .execute_proposal_transaction(&proposal_cookie, &proposal_transaction_cookie)
        .await;

    // Assert
    let extra_address =
        get_proposal_extra_account_address(&governance_test.program_id, &proposal_cookie.address);
    let extra_account = governance_test
        .bench
        .get_account(&extra_address)
        .await
        .unwrap();

    let expected_lamports = governance_test.bench.rent.minimum_balance(1);

    assert_eq!(extra_account.lamports, expected_lamports);
    assert_eq!(extra_account.owner, expected_owner);
    assert_eq!(extra_account.data, vec![0]);
}
