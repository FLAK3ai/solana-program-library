/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category Redeem
 * @category generated
 */
export type RedeemInstructionArgs = {
  root: number[] /* size: 32 */
  dataHash: number[] /* size: 32 */
  creatorHash: number[] /* size: 32 */
  nonce: beet.bignum
  index: number
}
/**
 * @category Instructions
 * @category Redeem
 * @category generated
 */
export const redeemStruct = new beet.BeetArgsStruct<
  RedeemInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['root', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['dataHash', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['creatorHash', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['nonce', beet.u64],
    ['index', beet.u32],
  ],
  'RedeemInstructionArgs'
)
/**
 * Accounts required by the _redeem_ instruction
 *
 * @property [] authority
 * @property [] candyWrapper
 * @property [] gummyrollProgram
 * @property [_writable_, **signer**] owner
 * @property [] delegate
 * @property [_writable_] merkleSlab
 * @property [_writable_] voucher
 * @category Instructions
 * @category Redeem
 * @category generated
 */
export type RedeemInstructionAccounts = {
  authority: web3.PublicKey
  candyWrapper: web3.PublicKey
  gummyrollProgram: web3.PublicKey
  owner: web3.PublicKey
  delegate: web3.PublicKey
  merkleSlab: web3.PublicKey
  voucher: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const redeemInstructionDiscriminator = [
  184, 12, 86, 149, 70, 196, 97, 225,
]

/**
 * Creates a _Redeem_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Redeem
 * @category generated
 */
export function createRedeemInstruction(
  accounts: RedeemInstructionAccounts,
  args: RedeemInstructionArgs,
  programId = new web3.PublicKey('BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY')
) {
  const [data] = redeemStruct.serialize({
    instructionDiscriminator: redeemInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.candyWrapper,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.gummyrollProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.owner,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.delegate,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.merkleSlab,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.voucher,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
