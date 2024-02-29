import type { ConfirmOptions, Connection, PublicKey, Signer, TransactionSignature } from '@solana/web3.js';
import { sendAndConfirmTransaction, Transaction } from '@solana/web3.js';
import {
    createInitializeGroupInstruction,
    createUpdateGroupMaxSizeInstruction,
    createUpdateGroupAuthorityInstruction,
    createInitializeMemberInstruction,
} from '@solana/spl-token-group';

import { TOKEN_2022_PROGRAM_ID } from '../../constants.js';
import { getSigners } from '../../actions/internal.js';

/**
 * Initialize a new `Group`
 *
 * Assumes one has already initialized a mint for the group.
 *
 * @param connection       Connection to use
 * @param payer            Payer of the transaction fees
 * @param mint             Mint Account
 * @param mintAuthority    Mint Authority
 * @param updateAuthority  Update Authority
 * @param maxSize          Maximum number of members in the group
 * @param multiSigners     Signing accounts if `authority` is a multisig
 * @param confirmOptions   Options for confirming the transaction
 * @param programId        SPL Token program account
 *
 * @return Signature of the confirmed transaction
 */
export async function tokenGroupInitializeGroup(
    connection: Connection,
    payer: Signer,
    mint: PublicKey,
    mintAuthority: PublicKey,
    updateAuthority: PublicKey | null,
    maxSize: number,
    multiSigners: Signer[] = [],
    confirmOptions?: ConfirmOptions,
    programId = TOKEN_2022_PROGRAM_ID
): Promise<TransactionSignature> {
    const [mintAuthorityPublicKey, signers] = getSigners(mintAuthority, multiSigners);

    const transaction = new Transaction().add(
        createInitializeGroupInstruction({
            programId,
            group: mint,
            mint,
            mintAuthority: mintAuthorityPublicKey,
            updateAuthority,
            maxSize,
        })
    );

    return await sendAndConfirmTransaction(connection, transaction, [payer, ...signers], confirmOptions);
}

/**
 * Update the max size of a `Group`
 *
 * @param connection       Connection to use
 * @param payer            Payer of the transaction fees
 * @param mint             Mint Account
 * @param updateAuthority  Update Authority
 * @param maxSize          Maximum number of members in the group
 * @param multiSigners     Signing accounts if `authority` is a multisig
 * @param confirmOptions   Options for confirming the transaction
 * @param programId        SPL Token program account
 *
 * @return Signature of the confirmed transaction
 */
export async function tokenGroupUpdateGroupMaxSize(
    connection: Connection,
    payer: Signer,
    mint: PublicKey,
    updateAuthority: PublicKey,
    maxSize: number,
    multiSigners: Signer[] = [],
    confirmOptions?: ConfirmOptions,
    programId = TOKEN_2022_PROGRAM_ID
): Promise<TransactionSignature> {
    const [updateAuthorityPublicKey, signers] = getSigners(updateAuthority, multiSigners);

    const transaction = new Transaction().add(
        createUpdateGroupMaxSizeInstruction({
            programId,
            group: mint,
            updateAuthority: updateAuthorityPublicKey,
            maxSize,
        })
    );

    return await sendAndConfirmTransaction(connection, transaction, [payer, ...signers], confirmOptions);
}

/**
 * Update the authority of a `Group`
 *
 * @param connection       Connection to use
 * @param payer            Payer of the transaction fees
 * @param mint             Mint Account
 * @param updateAuthority  Update Authority
 * @param newAuthority     New authority for the token group, or unset
 * @param multiSigners     Signing accounts if `authority` is a multisig
 * @param confirmOptions   Options for confirming the transaction
 * @param programId        SPL Token program account
 *
 * @return Signature of the confirmed transaction
 */
export async function tokenGroupUpdateGroupAuthority(
    connection: Connection,
    payer: Signer,
    mint: PublicKey,
    updateAuthority: PublicKey | Signer,
    newAuthority: PublicKey | null,
    multiSigners: Signer[] = [],
    confirmOptions?: ConfirmOptions,
    programId = TOKEN_2022_PROGRAM_ID
): Promise<TransactionSignature> {
    const [updateAuthorityPublicKey, signers] = getSigners(updateAuthority, multiSigners);

    const transaction = new Transaction().add(
        createUpdateGroupAuthorityInstruction({
            programId,
            group: mint,
            currentAuthority: updateAuthorityPublicKey,
            newAuthority,
        })
    );

    return await sendAndConfirmTransaction(connection, transaction, [payer, ...signers], confirmOptions);
}

/**
 * Initialize a new `Member` of a `Group`
 *
 * Assumes the `Group` has already been initialized,
 * as well as the mint for the member.
 *
 * @param connection             Connection to use
 * @param payer                  Payer of the transaction fees
 * @param member                 Member Account
 * @param memberMint             Mint Account for the member
 * @param memberMintAuthority    Mint Authority for the member
 * @param group                  Group Account
 * @param groupUpdateAuthority   Update Authority for the group
 * @param multiSigners           Signing accounts if `authority` is a multisig
 * @param confirmOptions         Options for confirming the transaction
 * @param programId              SPL Token program account
 *
 * @return Signature of the confirmed transaction
 */
export async function tokenGroupMemberInitialize(
    connection: Connection,
    payer: Signer,
    member: PublicKey,
    memberMint: PublicKey,
    memberMintAuthority: PublicKey,
    group: PublicKey,
    groupUpdateAuthority: PublicKey,
    multiSigners: Signer[] = [],
    confirmOptions?: ConfirmOptions,
    programId = TOKEN_2022_PROGRAM_ID
): Promise<TransactionSignature> {
    const [memberMintAuthorityPublicKey, signers] = getSigners(memberMintAuthority, multiSigners);

    const transaction = new Transaction().add(
        createInitializeMemberInstruction({
            programId,
            member,
            memberMint,
            memberMintAuthority: memberMintAuthorityPublicKey,
            group,
            groupUpdateAuthority,
        })
    );

    return await sendAndConfirmTransaction(connection, transaction, [payer, ...signers], confirmOptions);
}
