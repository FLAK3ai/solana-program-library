use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitializeBettingPoolArgs {
    pub decimals: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct TradeArgs {
    pub size: u64,
    pub buy_price: u64,
    pub sell_price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum BettingPoolInstruction {
    // TODO: Add comments here
    InitializeBettingPool(InitializeBettingPoolArgs),

    Trade(TradeArgs),

    Settle,

    Collect,
}

/// Creates an InitializeBettingPool instruction
#[allow(clippy::too_many_arguments)]
pub fn initailize_betting_pool(
    program_id: Pubkey,
    pool_account: Pubkey,
    escrow_mint: Pubkey,
    escrow_account: Pubkey,
    long_token_mint: Pubkey,
    short_token_mint: Pubkey,
    mint_authority: Pubkey,
    update_authority: Pubkey,
    decimals: u8,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, true),
            AccountMeta::new_readonly(escrow_mint, false),
            AccountMeta::new(escrow_account, true),
            AccountMeta::new_readonly(long_token_mint, true),
            AccountMeta::new_readonly(short_token_mint, true),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new_readonly(update_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: BettingPoolInstruction::InitializeBettingPool(InitializeBettingPoolArgs { decimals })
            .try_to_vec()
            .unwrap(),
    }
}

/// Creates a Trade instruction
#[allow(clippy::too_many_arguments)]
pub fn initailize_trade(
    program_id: Pubkey,
    pool_account: Pubkey,
    escrow_account: Pubkey,
    long_token_mint: Pubkey,
    short_token_mint: Pubkey,
    buyer: Pubkey,
    seller: Pubkey,
    buyer_account: Pubkey,
    seller_account: Pubkey,
    buyer_long_token_account: Pubkey,
    buyer_short_token_account: Pubkey,
    seller_long_token_account: Pubkey,
    seller_short_token_account: Pubkey,
    escrow_authority: Pubkey,
    size: u64,
    buy_price: u64,
    sell_price: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, false),
            AccountMeta::new(escrow_account, false),
            AccountMeta::new(long_token_mint, false),
            AccountMeta::new(short_token_mint, false),
            AccountMeta::new_readonly(buyer, true),
            AccountMeta::new_readonly(seller, true),
            AccountMeta::new(buyer_account, false),
            AccountMeta::new(seller_account, false),
            AccountMeta::new(buyer_long_token_account, false),
            AccountMeta::new(buyer_short_token_account, false),
            AccountMeta::new(seller_long_token_account, false),
            AccountMeta::new(seller_short_token_account, false),
            AccountMeta::new_readonly(escrow_authority, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: BettingPoolInstruction::Trade(TradeArgs { size, buy_price, sell_price })
            .try_to_vec()
            .unwrap(),
    }
}