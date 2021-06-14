import { Schema, serialize, deserializeUnchecked } from 'borsh';
import BN from 'bn.js';
import { SOLANA_SCHEMA, Struct, Enum } from '@solana/web3.js';

export class Fee extends Struct {
  denominator: BN;
  numerator: BN;
}

export class AccountType extends Enum { }

export class AccountTypeEnum extends Struct { }

export enum AccountTypeKind {
  Uninitialized = 'Uninitialized',
  StakePool = 'StakePool',
  ValidatorList = 'ValidatorList',
}

export class StakePool extends Struct {
  accountType: AccountType;
  manager: PublicKey;
  staker: PublicKey;
  depositAuthority: PublicKey;
  withdrawBumpSeed: number;
  validatorList: PublicKey;
  reserveStake: PublicKey;
  poolMint: PublicKey;
  managerFeeAccount: PublicKey;
  totalStakeLamports: BN;
  poolTokenSupply: BN;
  lastUpdateEpoch: BN;
  fee: Fee;
}

export class ValidatorList extends Struct {
  accountType: AccountType;
  maxValidators: number;
  validators: [ValidatorStakeInfo];
}
export class ValidatorStakeInfo extends Struct {
  status: StakeStatus;
  voteAccountAddress: PublicKey;
  stakeLamports: BN;
  lastUpdateEpoch: BN;
}
export class StakeStatus extends Enum { }

export class StakeStatusEnum extends Struct { }

export enum StakeStatusKind {
  Active = 'Active',
  DeactivatingTransient = 'DeactivatingTransient',
  ReadyForRemoval = 'ReadyForRemoval',
}

export class PublicKey extends Struct {
  value: BN;
}

export const SCHEMA: Schema = constructStakePoolSchema(SOLANA_SCHEMA);

export function constructStakePoolSchema(SCHEMA: Schema): Schema {
  /**
   * Borsh requires something called a Schema,
   * which is a Map (key-value pairs) that tell borsh how to deserialise the raw data
   * This function creates, populates and returns such a schema
   */
  SCHEMA.set(PublicKey, {
    kind: 'struct',
    fields: [['value', 'u256']],
  });

  SCHEMA.set(Fee, {
    kind: 'struct',
    fields: [
      ['denominator', 'u64'],
      ['numerator', 'u64'],
    ],
  });

  SCHEMA.set(AccountType, {
    kind: 'enum',
    field: 'enum',
    values: [
      // if the account has not been initialized, the enum will be 0
      [AccountTypeKind.Uninitialized, AccountTypeEnum],
      [AccountTypeKind.StakePool, AccountTypeEnum],
      [AccountTypeKind.ValidatorList, AccountTypeEnum],
    ],
  });

  SCHEMA.set(AccountTypeEnum, { kind: 'struct', fields: [] });

  SCHEMA.set(StakePool, {
    kind: 'struct',
    fields: [
      ['accountType', AccountType],
      ['manager', PublicKey],
      ['staker', PublicKey],
      ['depositAuthority', PublicKey],
      ['withdrawBumpSeed', 'u8'],
      ['validatorList', PublicKey],
      ['reserveStake', PublicKey],
      ['poolMint', PublicKey],
      ['managerFeeAccount', PublicKey],
      ['tokenProgramId', PublicKey],
      ['totalStakeLamports', 'u64'],
      ['poolTokenSupply', 'u64'],
      ['lastUpdateEpoch', 'u64'],
      ['fee', Fee],
    ],
  });

  SCHEMA.set(ValidatorList, {
    kind: 'struct',
    fields: [
      ['accountType', AccountType],
      ['maxValidators', 'u32'],
      ['validators', [ValidatorStakeInfo]],
    ],
  });

  SCHEMA.set(StakeStatus, {
    kind: 'enum',
    field: 'enum',
    values: [
      [StakeStatusKind.Active, StakeStatusEnum],
      [StakeStatusKind.DeactivatingTransient, StakeStatusEnum],
      [StakeStatusKind.ReadyForRemoval, StakeStatusEnum],
    ],
  });

  SCHEMA.set(StakeStatusEnum, { kind: 'struct', fields: [] });

  SCHEMA.set(ValidatorStakeInfo, {
    kind: 'struct',
    fields: [
      ['status', StakeStatus],
      ['voteAccountAddress', PublicKey],
      ['stakeLamports', 'u64'],
      ['lastUpdateEpoch', 'u64'],
    ],
  });

  return SCHEMA;
}
