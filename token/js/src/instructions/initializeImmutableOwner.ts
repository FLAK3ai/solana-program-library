import { struct, u8 } from '@solana/buffer-layout';
import { publicKey } from '@solana/buffer-layout-utils';
import { AccountMeta, PublicKey, TransactionInstruction } from '@solana/web3.js';
import {
    TokenInvalidInstructionDataError,
    TokenInvalidInstructionKeysError,
    TokenInvalidInstructionProgramError,
    TokenInvalidInstructionTypeError,
} from '../errors';
import { TokenInstruction } from './types';

/** TODO: docs */
export interface InitializeImmutableOwnerInstructionData {
    instruction: TokenInstruction.InitializeImmutableOwner;
}

/** TODO: docs */
export const initializeImmutableOwnerInstructionData = struct<InitializeImmutableOwnerInstructionData>([
    u8('instruction'),
]);

/**
 * Construct an InitializeImmutableOwner instruction
 *
 * @param account            Account
 * @param immutableOwner  Optional authority that can close the mint
 *
 * @return Instruction to add to a transaction
 */
export function createInitializeImmutableOwnerInstruction(
    account: PublicKey,
    programId: PublicKey
): TransactionInstruction {
    const keys = [{ pubkey: account, isSigner: false, isWritable: true }];

    const data = Buffer.alloc(initializeImmutableOwnerInstructionData.span);
    initializeImmutableOwnerInstructionData.encode(
        {
            instruction: TokenInstruction.InitializeImmutableOwner,
        },
        data
    );

    return new TransactionInstruction({ keys, programId, data });
}

/** A decoded, valid InitializeImmutableOwner instruction */
export interface DecodedInitializeImmutableOwnerInstruction {
    programId: PublicKey;
    keys: {
        account: AccountMeta;
    };
    data: {
        instruction: TokenInstruction.InitializeImmutableOwner;
    };
}

/**
 * Decode an InitializeImmutableOwner instruction and validate it
 *
 * @param instruction Transaction instruction to decode
 * @param programId   SPL Token program account
 *
 * @return Decoded, valid instruction
 */
export function decodeInitializeImmutableOwnerInstruction(
    instruction: TransactionInstruction,
    programId: PublicKey
): DecodedInitializeImmutableOwnerInstruction {
    if (!instruction.programId.equals(programId)) throw new TokenInvalidInstructionProgramError();
    if (instruction.data.length !== initializeImmutableOwnerInstructionData.span)
        throw new TokenInvalidInstructionDataError();

    const {
        keys: { account },
        data,
    } = decodeInitializeImmutableOwnerInstructionUnchecked(instruction);
    if (data.instruction !== TokenInstruction.InitializeImmutableOwner)
        throw new TokenInvalidInstructionTypeError();
    if (!account) throw new TokenInvalidInstructionKeysError();

    return {
        programId,
        keys: {
            account,
        },
        data,
    };
}

/** A decoded, non-validated InitializeImmutableOwner instruction */
export interface DecodedInitializeImmutableOwnerInstructionUnchecked {
    programId: PublicKey;
    keys: {
        account: AccountMeta | undefined;
    };
    data: {
        instruction: number;
    };
}

/**
 * Decode an InitializeImmutableOwner instruction without validating it
 *
 * @param instruction Transaction instruction to decode
 *
 * @return Decoded, non-validated instruction
 */
export function decodeInitializeImmutableOwnerInstructionUnchecked({
    programId,
    keys: [account],
    data,
}: TransactionInstruction): DecodedInitializeImmutableOwnerInstructionUnchecked {
    const { instruction } =
        initializeImmutableOwnerInstructionData.decode(data);

    return {
        programId,
        keys: {
            account: account,
        },
        data: {
            instruction
        },
    };
}