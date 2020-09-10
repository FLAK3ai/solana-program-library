// @flow

import fs from 'mz/fs';
import {Account, Connection, BpfLoader, PublicKey} from '@solana/web3.js';
import semver from 'semver';

import {Token, NATIVE_MINT} from '../client/token';
import {url} from '../url';
import {newAccountWithLamports} from '../client/util/new-account-with-lamports';
import {sleep} from '../client/util/sleep';
import {Store} from '../client/util/store';

// Loaded token program's program id
let programId: PublicKey;

// Accounts setup in createMint and used by all subsequent tests
let testMintAuthority: Account;
let testToken: Token;

// Accounts setup in createAccount and used by all subsequent tests
let testAccountOwner: Account;
let testAccount: PublicKey;

function assert(condition, message) {
  if (!condition) {
    console.log(Error().stack + ':token-test.js');
    throw message || 'Assertion failed';
  }
}

async function didThrow(func, args): Promise<boolean> {
  try {
    await func.apply(args);
  } catch (e) {
    return true;
  }
  return false;
}

let connection;
async function getConnection(): Promise<Connection> {
  if (connection) return connection;

  let newConnection = new Connection(url, 'recent');
  const version = await newConnection.getVersion();

  // commitment params are only supported >= 0.21.0
  const solanaCoreVersion = version['solana-core'].split(' ')[0];
  if (semver.gte(solanaCoreVersion, '0.21.0')) {
    newConnection = new Connection(url, 'recent');
  }

  // eslint-disable-next-line require-atomic-updates
  connection = newConnection;
  console.log('Connection to cluster established:', url, version);
  return connection;
}

async function loadProgram(
  connection: Connection,
  path: string,
): Promise<PublicKey> {
  const NUM_RETRIES = 500; /* allow some number of retries */
  const data = await fs.readFile(path);
  const {feeCalculator} = await connection.getRecentBlockhash();
  const balanceNeeded =
    feeCalculator.lamportsPerSignature *
      (BpfLoader.getMinNumSignatures(data.length) + NUM_RETRIES) +
    (await connection.getMinimumBalanceForRentExemption(data.length));

  const from = await newAccountWithLamports(connection, balanceNeeded);
  const program_account = new Account();
  console.log('Loading program:', path);
  await BpfLoader.load(connection, from, program_account, data, 2);
  return program_account.publicKey;
}

async function GetPrograms(connection: Connection): Promise<PublicKey> {
  const programVersion = process.env.PROGRAM_VERSION;
  if (programVersion) {
    switch (programVersion) {
      case '2.0.4':
        return new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
      default:
        throw new Error('Unknown program version');
    }
  }

  const store = new Store();
  let tokenProgramId = null;
  try {
    const config = await store.load('config.json');
    console.log('Using pre-loaded Token program');
    console.log(
      '  Note: To reload program remove client/util/store/config.json',
    );
    tokenProgramId = new PublicKey(config.tokenProgramId);
  } catch (err) {
    tokenProgramId = await loadProgram(
      connection,
      '../../target/bpfel-unknown-unknown/release/spl_token.so',
    );
    await store.save('config.json', {
      tokenProgramId: tokenProgramId.toString(),
    });
  }
  return tokenProgramId;
}

export async function loadTokenProgram(): Promise<void> {
  const connection = await getConnection();
  programId = await GetPrograms(connection);

  console.log('Token Program ID', programId.toString());
}

export async function createMint(): Promise<void> {
  const connection = await getConnection();
  const payer = await newAccountWithLamports(
    connection,
    100000000000 /* wag */,
  );
  testMintAuthority = new Account();
  testToken = await Token.createMint(
    connection,
    payer,
    testMintAuthority.publicKey,
    null,
    2,
    programId,
  );

  const mintInfo = await testToken.getMintInfo();
  if (mintInfo.mintAuthority !== null) {
    assert(mintInfo.mintAuthority.equals(testMintAuthority.publicKey));
  } else {
    assert(mintInfo.mintAuthority !== null);
  }
  assert(mintInfo.supply.toNumber() === 0);
  assert(mintInfo.decimals === 2);
  assert(mintInfo.isInitialized === true);
  assert(mintInfo.freezeAuthority === null);
}

export async function createAccount(): Promise<void> {
  testAccountOwner = new Account();
  testAccount = await testToken.createAccount(testAccountOwner.publicKey);
  const accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.mint.equals(testToken.publicKey));
  assert(accountInfo.owner.equals(testAccountOwner.publicKey));
  assert(accountInfo.amount.toNumber() === 0);
  assert(accountInfo.delegate === null);
  assert(accountInfo.delegatedAmount.toNumber() === 0);
  assert(accountInfo.isInitialized === true);
  assert(accountInfo.isFrozen === false);
  assert(accountInfo.isNative === false);
  assert(accountInfo.rentExemptReserve === null);
  assert(accountInfo.closeAuthority === null);
}

export async function mintTo(): Promise<void> {
  await testToken.mintTo(testAccount, testMintAuthority, [], 1000);

  const mintInfo = await testToken.getMintInfo();
  assert(mintInfo.supply.toNumber() === 1000);

  const accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.amount.toNumber() === 1000);
}

export async function transfer(): Promise<void> {
  const destOwner = new Account();
  const dest = await testToken.createAccount(destOwner.publicKey);

  await testToken.transfer(testAccount, dest, testAccountOwner, [], 100);

  const mintInfo = await testToken.getMintInfo();
  assert(mintInfo.supply.toNumber() === 1000);

  let destAccountInfo = await testToken.getAccountInfo(dest);
  assert(destAccountInfo.amount.toNumber() === 100);

  let testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.amount.toNumber() === 900);
}

export async function approveRevoke(): Promise<void> {
  const delegate = new Account().publicKey;

  await testToken.approve(testAccount, delegate, testAccountOwner, [], 42);

  let testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.delegatedAmount.toNumber() === 42);
  if (testAccountInfo.delegate === null) {
    throw new Error('delegate should not be null');
  } else {
    assert(testAccountInfo.delegate.equals(delegate));
  }

  await testToken.revoke(testAccount, testAccountOwner, []);

  testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.delegatedAmount.toNumber() === 0);
  if (testAccountInfo.delegate !== null) {
    throw new Error('delegate should be null');
  }
}

export async function invalidApprove(): Promise<void> {
  const owner = new Account();
  const account1 = await testToken.createAccount(owner.publicKey);
  const account2 = await testToken.createAccount(owner.publicKey);
  const delegate = new Account();

  // account2 is not a delegate account of account1
  assert(didThrow(testToken.approve, [account1, account2, owner, [], 123]));
  // account1Delegate is not a delegate account of account2
  assert(didThrow(testToken.approve, [account2, delegate, owner, [], 123]));
}

export async function failOnApproveOverspend(): Promise<void> {
  const owner = new Account();
  const account1 = await testToken.createAccount(owner.publicKey);
  const account2 = await testToken.createAccount(owner.publicKey);
  const delegate = new Account();

  await testToken.transfer(testAccount, account1, testAccountOwner, [], 10);

  await testToken.approve(account1, delegate.publicKey, owner, [], 2);

  let account1Info = await testToken.getAccountInfo(account1);
  assert(account1Info.amount.toNumber() == 10);
  assert(account1Info.delegatedAmount.toNumber() == 2);
  if (account1Info.delegate === null) {
    throw new Error('deleage should not be null');
  } else {
    assert(account1Info.delegate.equals(delegate.publicKey));
  }

  await testToken.transfer(account1, account2, delegate, [], 1);

  account1Info = await testToken.getAccountInfo(account1);
  assert(account1Info.amount.toNumber() == 9);
  assert(account1Info.delegatedAmount.toNumber() == 1);

  await testToken.transfer(account1, account2, delegate, [], 1);

  account1Info = await testToken.getAccountInfo(account1);
  assert(account1Info.amount.toNumber() == 8);
  assert(account1Info.delegate === null);
  assert(account1Info.delegatedAmount.toNumber() == 0);

  assert(didThrow(testToken.transfer, [account1, account2, delegate, [], 1]));
}

export async function setAuthority(): Promise<void> {
  const newOwner = new Account();
  await testToken.setAuthority(
    testAccount,
    newOwner.publicKey,
    'AccountOwner',
    testAccountOwner,
    [],
  );
  assert(
    didThrow(testToken.setAuthority, [
      testAccountOwner,
      newOwner.publicKey,
      'AccountOwner',
      testAccountOwner,
      [],
    ]),
  );
  await testToken.setAuthority(
    testAccount,
    testAccountOwner.publicKey,
    'AccountOwner',
    newOwner,
    [],
  );
}

export async function burn(): Promise<void> {
  let accountInfo = await testToken.getAccountInfo(testAccount);
  const amount = accountInfo.amount.toNumber();

  await testToken.burn(testAccount, testAccountOwner, [], 1);

  accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.amount.toNumber() == amount - 1);
}

export async function closeAccount(): Promise<void> {
  const closeAuthority = new Account();

  await testToken.setAuthority(
    testAccount,
    closeAuthority.publicKey,
    'CloseAccount',
    testAccountOwner,
    [],
  );
  let accountInfo = await testToken.getAccountInfo(testAccount);
  if (accountInfo.closeAuthority === null) {
    assert(accountInfo.closeAuthority !== null);
  } else {
    assert(accountInfo.closeAuthority.equals(closeAuthority.publicKey));
  }

  const dest = await testToken.createAccount(new Account().publicKey);
  const remaining = accountInfo.amount.toNumber();

  // Check that accounts with non-zero token balance cannot be closed
  assert(
    didThrow(testToken.closeAccount, [testAccount, dest, closeAuthority, []]),
  );

  const connection = await getConnection();
  let tokenRentExemptAmount;
  let info = await connection.getAccountInfo(testAccount);
  if (info != null) {
    tokenRentExemptAmount = info.lamports;
  } else {
    throw new Error('Account not found');
  }

  // Transfer away all tokens
  await testToken.transfer(testAccount, dest, testAccountOwner, [], remaining);

  // Close for real
  await testToken.closeAccount(testAccount, dest, closeAuthority, []);

  info = await connection.getAccountInfo(testAccount);
  assert(info === null);

  let destInfo = await connection.getAccountInfo(dest);
  if (destInfo !== null) {
    assert(destInfo.lamports === 2 * tokenRentExemptAmount);
  } else {
    throw new Error('Account not found');
  }

  let destAccountInfo = await testToken.getAccountInfo(dest);
  assert(destAccountInfo.amount.toNumber() === remaining);
}

export async function multisig(): Promise<void> {
  const m = 2;
  const n = 5;

  let signerAccounts = [];
  for (var i = 0; i < n; i++) {
    signerAccounts.push(new Account());
  }
  let signerPublicKeys = [];
  signerAccounts.forEach(account => signerPublicKeys.push(account.publicKey));
  const multisig = await testToken.createMultisig(m, signerPublicKeys);
  const multisigInfo = await testToken.getMultisigInfo(multisig);
  assert(multisigInfo.m === m);
  assert(multisigInfo.n === n);
  assert(multisigInfo.signer1.equals(signerPublicKeys[0]));
  assert(multisigInfo.signer2.equals(signerPublicKeys[1]));
  assert(multisigInfo.signer3.equals(signerPublicKeys[2]));
  assert(multisigInfo.signer4.equals(signerPublicKeys[3]));
  assert(multisigInfo.signer5.equals(signerPublicKeys[4]));

  const multisigOwnedAccount = await testToken.createAccount(multisig);
  const finalDest = await testToken.createAccount(multisig);

  await testToken.mintTo(multisigOwnedAccount, testMintAuthority, [], 1000);

  // Transfer via multisig
  await testToken.transfer(
    multisigOwnedAccount,
    finalDest,
    multisig,
    signerAccounts,
    1,
  );
  await sleep(500);
  let accountInfo = await testToken.getAccountInfo(finalDest);
  assert(accountInfo.amount.toNumber() == 1);

  // Approve via multisig
  {
    const delegate = new PublicKey();
    await testToken.approve(
      multisigOwnedAccount,
      delegate,
      multisig,
      signerAccounts,
      1,
    );
    const accountInfo = await testToken.getAccountInfo(multisigOwnedAccount);
    assert(accountInfo.delegate != null);
    if (accountInfo.delegate != null) {
      assert(accountInfo.delegate.equals(delegate));
      assert(accountInfo.delegatedAmount.toNumber() == 1);
    }
  }

  // SetAuthority of account via multisig
  {
    const newOwner = new PublicKey();
    await testToken.setAuthority(
      multisigOwnedAccount,
      newOwner,
      'AccountOwner',
      multisig,
      signerAccounts,
    );
    const accountInfo = await testToken.getAccountInfo(multisigOwnedAccount);
    assert(accountInfo.owner.equals(newOwner));
  }
}

export async function nativeToken(): Promise<void> {
  const connection = await getConnection();
  // this user both pays for the creation of the new token account
  // and provides the lamports to wrap
  const payer = await newAccountWithLamports(
    connection,
    100000000000 /* wag */,
  );
  const lamportsToWrap = 50000000000;

  const token = new Token(connection, NATIVE_MINT, programId, payer);
  const owner = new Account();
  const native = await Token.createWrappedNativeAccount(
    connection,
    programId,
    owner.publicKey,
    payer,
    lamportsToWrap,
  );
  let accountInfo = await token.getAccountInfo(native);
  assert(accountInfo.isNative);

  // check that the new account has wrapped native tokens.
  assert(accountInfo.amount.toNumber() === lamportsToWrap);

  let balance;
  let info = await connection.getAccountInfo(native);
  if (info != null) {
    balance = info.lamports;
  } else {
    throw new Error('Account not found');
  }

  const balanceNeeded = await connection.getMinimumBalanceForRentExemption(0);
  const dest = await newAccountWithLamports(connection, balanceNeeded);
  await token.closeAccount(native, dest.publicKey, owner, []);
  info = await connection.getAccountInfo(native);
  if (info != null) {
    throw new Error('Account not burned');
  }
  info = await connection.getAccountInfo(dest.publicKey);
  if (info != null) {
    assert(info.lamports == balanceNeeded + balance);
  } else {
    throw new Error('Account not found');
  }
}
