// program API, (de)serializing instruction data

use solana_program::{
	sysvar::rent,
	system_program,
	pubkey::Pubkey,
	instruction::{ Instruction, AccountMeta },
};
use borsh::{ BorshSerialize, BorshDeserialize };
use crate::state::NFTData;

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub struct CreateNFTDataAccountArgs {
	pub data: NFTData,
	pub id: u8
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub struct UpdateNFTPriceArgs {
	pub id: u8,
	pub price: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub struct PurchaseNFTArgs {
	pub id: u8,
	pub new_name: Option<String>,
	pub new_uri: Option<String>,
	pub new_price: Option<u64>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub enum NftInstruction {
	CreateNFTDataAccount(CreateNFTDataAccountArgs),
	UpdateNFTPrice(UpdateNFTPriceArgs),
	PurchaseNFT(PurchaseNFTArgs),
}

impl NftInstruction {

	pub fn create_nft_data_accounts(
		program_id: &Pubkey,
		nft_data_account: &Pubkey,
		payer: &Pubkey,
		id: u8,
		name: String,
		symbol: String,
		uri: String,
		last_price: u64,
		listed_price: u64,
		owner_address: &Pubkey,
	) -> Instruction {
		let account_metas = vec![
			AccountMeta::new(*nft_data_account, false),
			AccountMeta::new(*payer, true),
			AccountMeta::new_readonly(system_program::id(), false),
			AccountMeta::new_readonly(rent::id(), false)
		];

		let nft_data = Self::CreateNFTDataAccount(CreateNFTDataAccountArgs {
			data: NFTData {
				id,
				name,
				symbol,
				uri,
				last_price,
				listed_price,
				owner_address: *owner_address
			},
			id,
		});

		Instruction {
			program_id: *program_id,
			accounts: account_metas,
			data: nft_data.try_to_vec().unwrap()
		}
	}

	pub fn update_nft_price(
		program_id: &Pubkey,
		nft_data_account: &Pubkey,
		id: u8,
		new_price: u64,
		owner: &Pubkey,
		owner_nft_token_account: &Pubkey,
	) -> Instruction {
		let account_metas = vec![
			AccountMeta::new(*nft_data_account, false),
			AccountMeta::new_readonly(*owner, true),
			AccountMeta::new_readonly(*owner_nft_token_account, false)
		];

		let update_data = Self::UpdateNFTPrice(UpdateNFTPriceArgs {
			id,
			price: new_price
		});

		Instruction {
			program_id: *program_id,
			accounts: account_metas,
			data: update_data.try_to_vec().unwrap()
		}
	}

	pub fn purchase_nft(
        program_id: &Pubkey,
        nft_data_account: &Pubkey,
        id: u8,
        new_name: Option<String>,
        new_uri: Option<String>,
        new_price: Option<u64>,
        nft_owner_address: &Pubkey,
        nft_token_account: &Pubkey,
	) -> Instruction {
		let account_metas = vec![
            AccountMeta::new(*nft_data_account, false),
            AccountMeta::new_readonly(*nft_owner_address, true),
            AccountMeta::new_readonly(*nft_token_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(rent::id(), false),
		];

		let purchase_data = Self::PurchaseNFT(PurchaseNFTArgs {
            id,
            new_name,
            new_uri,
            new_price,
		});

		Instruction {
			program_id: *program_id,
			accounts: account_metas,
			data: purchase_data.try_to_vec().unwrap()
		}
	}
}