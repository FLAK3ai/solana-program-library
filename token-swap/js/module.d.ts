declare module '@solana/spl-token-swap' {
  import { Buffer } from 'buffer';
  import { PublicKey, TransactionInstruction, TransactionSignature, Connection } from "@solana/web3.js";
  import BN from 'bn.js';

  // === client/token-swap.js ===
  export class Numberu64 extends BN {
    toBuffer(): Buffer;
    static fromBuffer(buffer: Buffer): Numberu64;
  }

  export type TokenSwapInfo = {
    nonce: number,
    tokenAccountA: PublicKey,
    tokenAccountB: PublicKey,
    tokenPool: PublicKey,
    feesNumerator: Numberu64,
    feesDenominator: Numberu64,
    feeRatio: number,
  };

  export class TokenSwap {
    constructor(connection: Connection, tokenSwap: PublicKey, programId: PublicKey, payer: Account);

    static getMinBalanceRentForExemptTokenSwap(
      connection: Connection,
    ): Promise<number>;

    static createTokenSwap(
      connection: Connection,
      payer: Account,
      tokenSwapAccount: Account,
      authority: PublicKey,
      tokenAccountA: PublicKey,
      tokenAccountB: PublicKey,
      tokenPool: PublicKey,
      tokenAccountPool: PublicKey,
      tokenProgramId: PublicKey,
      nonce: number,
      feeNumerator: number,
      feeDenominator: number,
      programId: PublicKey,
    ): Promise<TokenSwap>

    getInfo(): Promise<TokenSwapInfo>
    swap(
      authority: PublicKey,
      source: PublicKey,
      swap_source: PublicKey,
      swap_destination: PublicKey,
      destination: PublicKey,
      tokenProgramId: PublicKey,
      amount: number | Numberu64,
    ): Promise<TransactionSignature>

    swapInstruction(
      authority: PublicKey,
      source: PublicKey,
      swap_source: PublicKey,
      swap_destination: PublicKey,
      destination: PublicKey,
      tokenProgramId: PublicKey,
      amount: number | Numberu64,
    ): TransactionInstruction

    deposit(
      authority: PublicKey,
      sourceA: PublicKey,
      sourceB: PublicKey,
      intoA: PublicKey,
      intoB: PublicKey,
      poolToken: PublicKey,
      poolAccount: PublicKey,
      tokenProgramId: PublicKey,
      amount: number | Numberu64,
    ): Promise<TransactionSignature>

    depositInstruction(
      authority: PublicKey,
      sourceA: PublicKey,
      sourceB: PublicKey,
      intoA: PublicKey,
      intoB: PublicKey,
      poolToken: PublicKey,
      poolAccount: PublicKey,
      tokenProgramId: PublicKey,
      amount: number | Numberu64,
    ): TransactionInstruction

    withdraw(
      authority: PublicKey,
      poolMint: PublicKey,
      sourcePoolAccount: PublicKey,
      fromA: PublicKey,
      fromB: PublicKey,
      userAccountA: PublicKey,
      userAccountB: PublicKey,
      tokenProgramId: PublicKey,
      amount: number | Numberu64,
    ): Promise<TransactionSignature>

    withdrawInstruction(
      authority: PublicKey,
      poolMint: PublicKey,
      sourcePoolAccount: PublicKey,
      fromA: PublicKey,
      fromB: PublicKey,
      userAccountA: PublicKey,
      userAccountB: PublicKey,
      tokenProgramId: PublicKey,
      amount: number | Numberu64,
    ): TransactionInstruction
  }
}
