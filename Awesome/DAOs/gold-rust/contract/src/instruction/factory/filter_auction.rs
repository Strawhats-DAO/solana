use super::*;

pub fn filter_auction(admin_pubkey: Pubkey, auction_id: AuctionId, filter: bool) -> Instruction {
    let (auction_root_state_pubkey, _) =
        Pubkey::find_program_address(&auction_root_state_seeds(&auction_id), &crate::ID);
    let (contract_bank_pubkey, _) =
        Pubkey::find_program_address(&contract_bank_seeds(), &crate::ID);
    let (auction_pool_pubkey, _) = Pubkey::find_program_address(&auction_pool_seeds(), &crate::ID);
    let (secondary_pool_pubkey, _) =
        Pubkey::find_program_address(&secondary_pool_seeds(), &crate::ID);

    let accounts = vec![
        AccountMeta::new_readonly(admin_pubkey, true),
        AccountMeta::new(auction_root_state_pubkey, false),
        AccountMeta::new_readonly(contract_bank_pubkey, false),
        AccountMeta::new(auction_pool_pubkey, false),
        AccountMeta::new(secondary_pool_pubkey, false),
    ];

    let instruction = AuctionInstruction::FilterAuction {
        id: auction_id,
        filter,
    };

    Instruction {
        program_id: crate::ID,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}
