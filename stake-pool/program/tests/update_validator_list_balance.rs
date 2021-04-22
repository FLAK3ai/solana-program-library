#![cfg(feature = "test-bpf")]

mod helpers;

use {
    borsh::BorshDeserialize,
    helpers::*,
    solana_program::pubkey::Pubkey,
    solana_program_test::*,
    solana_sdk::signature::Signer,
    spl_stake_pool::{
        stake_program, state::StakePool, MAX_VALIDATORS_TO_UPDATE, MINIMUM_ACTIVE_STAKE,
    },
};

async fn setup(
    num_validators: usize,
) -> (
    ProgramTestContext,
    StakePoolAccounts,
    Vec<ValidatorStakeAccount>,
    u64,
    u64,
) {
    let mut context = program_test().start_with_context().await;
    let first_normal_slot = context.genesis_config().epoch_schedule.first_normal_slot;
    let slots_per_epoch = context.genesis_config().epoch_schedule.slots_per_epoch;
    let mut slot = first_normal_slot;
    context.warp_to_slot(slot).unwrap();

    let stake_pool_accounts = StakePoolAccounts::new();
    stake_pool_accounts
        .initialize_stake_pool(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            TEST_STAKE_AMOUNT + 1,
        )
        .await
        .unwrap();

    // Add several accounts with some stake
    let mut stake_accounts: Vec<ValidatorStakeAccount> = vec![];
    let mut deposit_accounts: Vec<DepositStakeAccount> = vec![];
    for _ in 0..num_validators {
        let stake_account = ValidatorStakeAccount::new(&stake_pool_accounts.stake_pool.pubkey());
        stake_account
            .create_and_delegate(
                &mut context.banks_client,
                &context.payer,
                &context.last_blockhash,
                &stake_pool_accounts.staker,
            )
            .await;

        let deposit_account = DepositStakeAccount::new_with_vote(
            stake_account.vote.pubkey(),
            stake_account.stake_account,
            TEST_STAKE_AMOUNT,
        );
        deposit_account
            .create_and_delegate(
                &mut context.banks_client,
                &context.payer,
                &context.last_blockhash,
            )
            .await;

        stake_accounts.push(stake_account);
        deposit_accounts.push(deposit_account);
    }

    // Warp forward so the stakes properly activate, and deposit
    // TODO This is *bad* -- program-test needs to have some more active stake
    // so we can warm up faster than this. Also, we need to do each of these
    // warps by hand to get fully active stakes.
    for i in 2..50 {
        slot = first_normal_slot + i * slots_per_epoch;
        context.warp_to_slot(slot).unwrap();
    }

    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;

    for stake_account in &stake_accounts {
        let error = stake_pool_accounts
            .add_validator_to_pool(
                &mut context.banks_client,
                &context.payer,
                &context.last_blockhash,
                &stake_account.stake_account,
            )
            .await;
        assert!(error.is_none());
    }

    for deposit_account in &deposit_accounts {
        deposit_account
            .deposit(
                &mut context.banks_client,
                &context.payer,
                &context.last_blockhash,
                &stake_pool_accounts,
            )
            .await;
    }

    slot += slots_per_epoch;
    context.warp_to_slot(slot).unwrap();

    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;

    (
        context,
        stake_pool_accounts,
        stake_accounts,
        TEST_STAKE_AMOUNT,
        slot,
    )
}

#[tokio::test]
#[ignore]
async fn success() {
    let num_validators = 5;
    let (mut context, stake_pool_accounts, stake_accounts, _, mut slot) =
        setup(num_validators).await;

    // Check current balance in the list
    let rent = context.banks_client.get_rent().await.unwrap();
    let stake_rent = rent.minimum_balance(std::mem::size_of::<stake_program::StakeState>());
    // initially, have all of the deposits plus their rent, and the reserve stake
    let initial_lamports =
        (TEST_STAKE_AMOUNT + stake_rent) * num_validators as u64 + TEST_STAKE_AMOUNT;
    assert_eq!(
        get_validator_list_sum(
            &mut context.banks_client,
            &stake_pool_accounts.reserve_stake.pubkey(),
            &stake_pool_accounts.validator_list.pubkey()
        )
        .await,
        initial_lamports,
    );

    // Simulate rewards
    for stake_account in &stake_accounts {
        context.increment_vote_account_credits(&stake_account.vote.pubkey(), 100);
    }

    // Warp one more epoch so the rewards are paid out
    let slots_per_epoch = context.genesis_config().epoch_schedule.slots_per_epoch;
    slot += slots_per_epoch;
    context.warp_to_slot(slot).unwrap();

    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;
    let new_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;
    assert!(new_lamports > initial_lamports);

    let stake_pool_info = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = StakePool::try_from_slice(&stake_pool_info.data).unwrap();
    assert_eq!(new_lamports, stake_pool.total_stake_lamports);
}

#[tokio::test]
#[ignore]
async fn merge_into_reserve() {
    let (mut context, stake_pool_accounts, stake_accounts, lamports, mut slot) =
        setup(MAX_VALIDATORS_TO_UPDATE).await;

    let pre_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;

    let reserve_stake = context
        .banks_client
        .get_account(stake_pool_accounts.reserve_stake.pubkey())
        .await
        .unwrap()
        .unwrap();
    let pre_reserve_lamports = reserve_stake.lamports;

    // Decrease from all validators
    for stake_account in &stake_accounts {
        let error = stake_pool_accounts
            .decrease_validator_stake(
                &mut context.banks_client,
                &context.payer,
                &context.last_blockhash,
                &stake_account.stake_account,
                &stake_account.transient_stake_account,
                lamports,
            )
            .await;
        assert!(error.is_none());
    }

    // Update, should not change, no merges yet
    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;

    let expected_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;
    assert_eq!(pre_lamports, expected_lamports);

    let stake_pool_info = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = StakePool::try_from_slice(&stake_pool_info.data).unwrap();
    assert_eq!(expected_lamports, stake_pool.total_stake_lamports);

    // Warp one more epoch so the stakes deactivate
    let slots_per_epoch = context.genesis_config().epoch_schedule.slots_per_epoch;
    slot += slots_per_epoch;
    context.warp_to_slot(slot).unwrap();

    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;
    let expected_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;
    assert_eq!(pre_lamports, expected_lamports);

    let reserve_stake = context
        .banks_client
        .get_account(stake_pool_accounts.reserve_stake.pubkey())
        .await
        .unwrap()
        .unwrap();
    let post_reserve_lamports = reserve_stake.lamports;
    assert!(post_reserve_lamports > pre_reserve_lamports);

    let stake_pool_info = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = StakePool::try_from_slice(&stake_pool_info.data).unwrap();
    assert_eq!(expected_lamports, stake_pool.total_stake_lamports);
}

#[tokio::test]
#[ignore]
async fn merge_into_validator_stake() {
    let (mut context, stake_pool_accounts, stake_accounts, lamports, mut slot) =
        setup(MAX_VALIDATORS_TO_UPDATE).await;

    let rent = context.banks_client.get_rent().await.unwrap();
    let pre_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;

    // Increase stake to all validators
    for stake_account in &stake_accounts {
        let error = stake_pool_accounts
            .increase_validator_stake(
                &mut context.banks_client,
                &context.payer,
                &context.last_blockhash,
                &stake_account.transient_stake_account,
                &stake_account.vote.pubkey(),
                lamports / stake_accounts.len() as u64,
            )
            .await;
        assert!(error.is_none());
    }

    // Warp just a little bit to get a new blockhash and update again
    context.warp_to_slot(slot + 10).unwrap();

    // Update, should not change, no merges yet
    let error = stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;
    assert!(error.is_none());

    let expected_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;
    assert_eq!(pre_lamports, expected_lamports);
    let stake_pool_info = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = StakePool::try_from_slice(&stake_pool_info.data).unwrap();
    assert_eq!(expected_lamports, stake_pool.total_stake_lamports);

    let stake_pool_info = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = StakePool::try_from_slice(&stake_pool_info.data).unwrap();
    assert_eq!(expected_lamports, stake_pool.total_stake_lamports);

    // Warp one more epoch so the stakes activate, ready to merge
    let slots_per_epoch = context.genesis_config().epoch_schedule.slots_per_epoch;
    slot += slots_per_epoch;
    context.warp_to_slot(slot).unwrap();

    let error = stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            stake_accounts
                .iter()
                .map(|v| v.vote.pubkey())
                .collect::<Vec<Pubkey>>()
                .as_slice(),
            false,
        )
        .await;
    assert!(error.is_none());
    let current_lamports = get_validator_list_sum(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
    )
    .await;
    let stake_pool_info = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = StakePool::try_from_slice(&stake_pool_info.data).unwrap();
    assert_eq!(current_lamports, stake_pool.total_stake_lamports);

    // Check that transient accounts are gone
    for stake_account in &stake_accounts {
        assert!(context
            .banks_client
            .get_account(stake_account.transient_stake_account)
            .await
            .unwrap()
            .is_none());
    }

    // Check validator stake accounts have the expected balance now:
    // validator stake account minimum + deposited lamports + 2 rents + increased lamports
    let expected_lamports = MINIMUM_ACTIVE_STAKE
        + lamports
        + lamports / stake_accounts.len() as u64
        + 2 * rent.minimum_balance(std::mem::size_of::<stake_program::StakeState>());
    for stake_account in &stake_accounts {
        let validator_stake =
            get_account(&mut context.banks_client, &stake_account.stake_account).await;
        assert_eq!(validator_stake.lamports, expected_lamports);
    }
}

#[tokio::test]
async fn max_validators() {}

#[tokio::test]
async fn fail_with_uninitialized_validator_list() {} // TODO

#[tokio::test]
async fn fail_with_wrong_stake_state() {} // TODO
