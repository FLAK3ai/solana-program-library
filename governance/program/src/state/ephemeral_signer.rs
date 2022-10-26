//! Ephemeral signer
use solana_program::pubkey::Pubkey;
use std::convert::TryFrom;


use super::proposal_transaction::{ProposalTransactionV2, SignerType};

/// TO DO DOCUMENTATION
pub fn get_ephemeral_signer_seeds<'a>(proposal_transaction_pubkey: &'a Pubkey, account_seq_number_le_bytes : &'a [u8; 2]) -> [&'a [u8]; 3] {
    [b"ephemeral-signer", proposal_transaction_pubkey.as_ref(), account_seq_number_le_bytes]
}

/// Returns ProposalExtraAccount PDA address
pub fn get_ephemeral_signer_address_and_seeds<'a>(program_id: &Pubkey, proposal_transaction_pubkey: &'a Pubkey, account_seq_number_le_bytes : &'a [u8; 2]) -> (Pubkey, u8, Vec<&'a [u8]>)  {
    let seeds = &get_ephemeral_signer_seeds(proposal_transaction_pubkey, account_seq_number_le_bytes);
    let (address, bump) = Pubkey::find_program_address(seeds, program_id);
    let seeds_vec = seeds.to_vec();
    return (address, bump, seeds_vec)
}

/// Returns ProposalExtraAccount PDA address
pub fn get_ephemeral_signer_address(program_id: &Pubkey, proposal_transaction_pubkey: &Pubkey, account_seq_number_le_bytes : &[u8; 2]) -> Pubkey  {
    let seeds = &get_ephemeral_signer_seeds(proposal_transaction_pubkey, &account_seq_number_le_bytes);
    Pubkey::find_program_address(seeds, program_id).0
}

 /// DOCS
pub struct EphemeralSeedGenerator {
     /// DOCS
    pub account_seq_numbers : Vec<[u8;2]>,
     /// DOCS
    pub bump_seeds : Vec<[u8;1]>,
}

impl EphemeralSeedGenerator {
     /// DOCS
    pub fn new(proposal_transaction_data : &ProposalTransactionV2 ) -> Self{
        let number_of_ephemeral_accounts : usize = proposal_transaction_data.instructions.iter().map(|ix| &ix.accounts).flatten().filter(|acc| acc.is_signer == SignerType::Ephemeral).count();

        EphemeralSeedGenerator {
            account_seq_numbers : (0..number_of_ephemeral_accounts).map(|x| u16::try_from(x).unwrap().to_le_bytes()).collect(),
            bump_seeds : vec![],
        }
    }

    /// DOCS
    pub fn generate<'a>(&'a mut self, program_id : &Pubkey, proposal_transaction_pubkey : &'a Pubkey, proposal_transaction_data : &ProposalTransactionV2) -> Vec<[&'a [u8];4]>{
        let mut ephemeral_signer_seeds = vec![];
        let mut i = 0usize;
        for instruction in proposal_transaction_data.instructions.iter() {
            for account in instruction.accounts.iter(){
                if account.is_signer == SignerType::Ephemeral {
                    let seeds : [&[u8];3] = get_ephemeral_signer_seeds(proposal_transaction_pubkey,  &self.account_seq_numbers[i]);
                    let (_, bump) = Pubkey::find_program_address(&seeds, program_id);
                    self.bump_seeds.push([bump]);
                    ephemeral_signer_seeds.push(seeds);
                    i = i.checked_add(1).unwrap();
                }
            }
        }

        let mut signers_seeds = vec![];
        for (seeds, bump) in ephemeral_signer_seeds.iter().zip(self.bump_seeds.iter()) {
            signers_seeds.push([seeds[0], seeds[1], seeds[2], bump]);
        }

        signers_seeds
    }


}

