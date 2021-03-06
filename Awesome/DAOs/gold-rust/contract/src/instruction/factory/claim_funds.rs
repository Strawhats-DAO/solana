use super::*;

#[derive(BorshSchema, BorshSerialize, BorshDeserialize)]
pub struct ClaimFundsArgs {
    pub payer_pubkey: Pubkey,
    pub auction_owner_pubkey: Pubkey,
    #[alias([u8; 32])]
    pub auction_id: AuctionId,
    pub cycle_number: u64,
    pub amount: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrontendClaimFundsArgs {
    pub payer_pubkey: String,
    pub auction_owner_pubkey: String,
    pub auction_id: String,
    pub cycle_number: u64,
    pub amount: Scalar,
}

impl TryFrom<FrontendClaimFundsArgs> for ClaimFundsArgs {
    type Error = String;
    fn try_from(args: FrontendClaimFundsArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            payer_pubkey: Pubkey::from_str(&args.payer_pubkey).map_err(|e| e.to_string())?,
            auction_owner_pubkey: Pubkey::from_str(&args.auction_owner_pubkey)
                .map_err(|e| e.to_string())?,
            auction_id: pad_to_32_bytes(&args.auction_id)?,
            cycle_number: args.cycle_number,
            amount: to_lamports(args.amount),
        })
    }
}

pub fn claim_funds(args: &ClaimFundsArgs) -> Instruction {
    let (auction_bank_pubkey, _) =
        Pubkey::find_program_address(&auction_bank_seeds(&args.auction_id), &crate::ID);
    let (auction_root_state_pubkey, _) =
        Pubkey::find_program_address(&auction_root_state_seeds(&args.auction_id), &crate::ID);
    let (auction_cycle_state_pubkey, _) = Pubkey::find_program_address(
        &auction_cycle_state_seeds(&auction_root_state_pubkey, &args.cycle_number.to_le_bytes()),
        &crate::ID,
    );

    let (contract_bank_pubkey, _) =
        Pubkey::find_program_address(&contract_bank_seeds(), &crate::ID);

    let (protocol_fee_state_pubkey, _) =
        Pubkey::find_program_address(&protocol_fee_state_seeds(), &crate::ID);

    let accounts = vec![
        AccountMeta::new(args.payer_pubkey, true),
        AccountMeta::new(args.auction_owner_pubkey, false),
        AccountMeta::new(auction_bank_pubkey, false),
        AccountMeta::new(auction_root_state_pubkey, false),
        AccountMeta::new(auction_cycle_state_pubkey, false),
        AccountMeta::new(contract_bank_pubkey, false),
        AccountMeta::new_readonly(protocol_fee_state_pubkey, false),
    ];

    let instruction = AuctionInstruction::ClaimFunds {
        id: args.auction_id,
        amount: args.amount,
    };

    Instruction {
        program_id: crate::ID,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}
