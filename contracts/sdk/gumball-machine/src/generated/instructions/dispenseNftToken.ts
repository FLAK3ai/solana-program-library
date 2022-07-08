/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category DispenseNftToken
 * @category generated
 */
export type DispenseNftTokenInstructionArgs = {
  numItems: number
}
/**
 * @category Instructions
 * @category DispenseNftToken
 * @category generated
 */
export const dispenseNftTokenStruct = new beet.BeetArgsStruct<
  DispenseNftTokenInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['numItems', beet.u32],
  ],
  'DispenseNftTokenInstructionArgs'
)
/**
 * Accounts required by the _dispenseNftToken_ instruction
 *
 * @property [_writable_] gumballMachine
 * @property [**signer**] payer
 * @property [_writable_] payerTokens
 * @property [_writable_] receiver
 * @property [] willyWonka
 * @property [] recentBlockhashes
 * @property [] instructionSysvarAccount
 * @property [_writable_] bubblegumAuthority
 * @property [] candyWrapper
 * @property [] gummyroll
 * @property [_writable_] merkleSlab
 * @property [] bubblegum
 * @category Instructions
 * @category DispenseNftToken
 * @category generated
 */
export type DispenseNftTokenInstructionAccounts = {
  gumballMachine: web3.PublicKey
  payer: web3.PublicKey
  payerTokens: web3.PublicKey
  receiver: web3.PublicKey
  willyWonka: web3.PublicKey
  recentBlockhashes: web3.PublicKey
  instructionSysvarAccount: web3.PublicKey
  bubblegumAuthority: web3.PublicKey
  candyWrapper: web3.PublicKey
  gummyroll: web3.PublicKey
  merkleSlab: web3.PublicKey
  bubblegum: web3.PublicKey
}

export const dispenseNftTokenInstructionDiscriminator = [
  55, 215, 72, 66, 100, 249, 57, 225,
]

/**
 * Creates a _DispenseNftToken_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category DispenseNftToken
 * @category generated
 */
export function createDispenseNftTokenInstruction(
  accounts: DispenseNftTokenInstructionAccounts,
  args: DispenseNftTokenInstructionArgs
) {
  const {
    gumballMachine,
    payer,
    payerTokens,
    receiver,
    willyWonka,
    recentBlockhashes,
    instructionSysvarAccount,
    bubblegumAuthority,
    candyWrapper,
    gummyroll,
    merkleSlab,
    bubblegum,
  } = accounts

  const [data] = dispenseNftTokenStruct.serialize({
    instructionDiscriminator: dispenseNftTokenInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: gumballMachine,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: payer,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: payerTokens,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: receiver,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: willyWonka,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: recentBlockhashes,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: instructionSysvarAccount,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: bubblegumAuthority,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: candyWrapper,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: gummyroll,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: merkleSlab,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: bubblegum,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey(
      'GBALLoMcmimUutWvtNdFFGH5oguS7ghUUV6toQPppuTW'
    ),
    keys,
    data,
  })
  return ix
}
