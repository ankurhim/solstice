// program logic

use crate::{
	state::{ NFTData, PREFIX, MAX_DATA_SIZE, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH },
	instruction::NftInstruction,
	error::CustomError,
};
use borsh::{ BorshSerialize, BorshDeserialize };
use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        program::{ invoke, invoke_signed },
        sysvar::rent::Rent,
        sysvar::Sysvar,
        system_instruction,
        program_pack::{IsInitialized, Pack},
};
use spl_token::state::Account;

pub struct Processor;

impl Processor {
	pub fn process_instruction(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		instruction_data: &[u8]
	) -> ProgramResult {
		let instruction = NftInstruction::try_from_slice(instruction_data)?;

		match instruction {
			NftInstruction::CreateNFTDataAccount(args) => {
				msg!("Instruction: Create nft Data Accounts");
				process_create_nft_data_accounts(
					program_id,
			                accounts,
			                args.data,
							args.id
			        )
			},
			NftInstruction::UpdateNFTPrice(args) => {
				msg!("Instruction: Update NFT Price from Id");
				process_update_nft_price(
						program_id,
						accounts,
						args.id,
						args.price,
				)
			},
			NftInstruction::PurchaseNFT(args) => {
				msg!("Instruction: Purchase NFT from Id");
				process_purchase_nft(
					program_id,
					accounts,
					args.id,
					args.new_name,
					args.new_uri,
					args.new_price,
				)
			}

		}
	}
}

pub fn process_create_nft_data_accounts(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	data: NFTData,
	id: u8,
) -> ProgramResult {
	let account_iter = &mut accounts.iter();

	let nft_data_account_info = next_account_info(account_iter)?;
	let payer_info = next_account_info(account_iter)?;
	let system_account_info = next_account_info(account_iter)?;
	let rent_account_info = next_account_info(account_iter)?;

	let nft_data_seeds = &[
		PREFIX.as_bytes(),
		program_id.as_ref(),
		&[id]
	];

	let (nft_data_key, nft_data_bump_seed) = Pubkey::find_program_address(nft_data_seeds, program_id);
	msg!("nft Data Key: {:?}", &nft_data_key);

	let nft_data_authority_signer_seeds = &[
		PREFIX.as_bytes(),
        	program_id.as_ref(),
        	&[id],
        	&[nft_data_bump_seed],
    	];

    	if *nft_data_account_info.key != nft_data_key {
    		return Err(CustomError::InvalidNFTDataKey.into());
    	}

    	let rent = &Rent::from_account_info(rent_account_info)?;
    	let req_lamports = rent.minimum_balance(MAX_DATA_SIZE).max(1).saturating_sub(nft_data_account_info.lamports());

    	if req_lamports > 0 {
    		msg!("--> {} lamports are transferred to the new acccount", req_lamports);
    		invoke(
    			&system_instruction::transfer(&payer_info.key, nft_data_account_info.key, req_lamports),
    			&[
    			payer_info.clone(),
    			nft_data_account_info.clone(),
    			system_account_info.clone()
    			],
    		)?;
    	}

    	let accounts = &[nft_data_account_info.clone(), system_account_info.clone()];

    	msg!("--> Allocate space for the account.");
    	invoke_signed(
    		&system_instruction::allocate(nft_data_account_info.key, MAX_DATA_SIZE.try_into().unwrap()),
    		accounts,
    		&[nft_data_authority_signer_seeds],
    	)?;

		msg!("--> Assign the account to the owning program");
		invoke_signed(
			&system_instruction::assign(nft_data_account_info.key, &program_id),
			accounts,
			&[nft_data_authority_signer_seeds],
		)?;

    	let mut nft_data = NFTData::from_acc_info(nft_data_account_info)?;

    	if data.name.len() > MAX_NAME_LENGTH {
    		return Err(CustomError::NameTooLong.into());
    	}

		if data.symbol.len() > MAX_SYMBOL_LENGTH {
    		return Err(CustomError::SymbolTooLong.into());
    	}

    	if data.uri.len() > MAX_URI_LENGTH {
    		return Err(CustomError::UriTooLong.into());
    	}

    	nft_data.id = data.id;
    	nft_data.name = data.name;
		nft_data.symbol = data.symbol;
    	nft_data.uri = data.uri;
    	nft_data.last_price = data.last_price;
    	nft_data.listed_price = data.listed_price;
    	nft_data.owner_address = data.owner_address;

    	let mut array_of_zeroes = vec![];

    	while array_of_zeroes.len() > MAX_NAME_LENGTH - nft_data.name.len() {
    		array_of_zeroes.push(0u8);
    	}

    	nft_data.name = nft_data.name.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();

		let mut array_of_zeroes = vec![];

    	while array_of_zeroes.len() > MAX_SYMBOL_LENGTH - nft_data.symbol.len() {
    		array_of_zeroes.push(0u8);
    	}

    	nft_data.symbol = nft_data.symbol.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();

    	let mut array_of_zeroes = vec![];

    	while array_of_zeroes.len() > MAX_URI_LENGTH - nft_data.uri.len() {
    		array_of_zeroes.push(0u8);
    	}

    	nft_data.uri = nft_data.uri.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();

    	nft_data.serialize(&mut *nft_data_account_info.data.borrow_mut())?;
    	msg!("nft Data Saved! {:#?}", nft_data);

	Ok(())
}

pub fn process_update_nft_price(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	nft_id: u8,
	new_price: u64
) -> ProgramResult {
	let account_iter = &mut accounts.iter();

	let nft_data_account_info = next_account_info(account_iter)?;
	let owner_info = next_account_info(account_iter)?;
	let owner_nft_account_info = next_account_info(account_iter)?;

	let nft_data_seeds = &[
		PREFIX.as_bytes(),
		program_id.as_ref(),
		&[nft_id]
	];

	let (nft_data_key, _nft_data_bump_seed) = Pubkey::find_program_address(nft_data_seeds, program_id);

	if *nft_data_account_info.key != nft_data_key {
		return Err(CustomError::InvalidNFTDataKey.into());
	}

	if nft_data_account_info.owner != program_id {
		return Err(CustomError::IncorrectOwner.into());
	}

	let mut nft_data = NFTData::from_acc_info(nft_data_account_info)?;

	let token_acc: Account = assert_initialized(&owner_nft_account_info)?;

	if owner_nft_account_info.owner != &spl_token::id() {
		return Err(CustomError::IncorrectOwner.into());
	};

	if nft_data.owner_address != token_acc.mint {
		return Err(CustomError::OwnerMismatch.into());
	}

	if token_acc.owner != *owner_info.key {
        	return Err(CustomError::InvalidOwner.into());
    	}

    	nft_data.listed_price = new_price;

    	nft_data.serialize(&mut *nft_data_account_info.data.borrow_mut())?;

	Ok(())
}

pub fn process_purchase_nft(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	id: u8,
	new_name: Option<String>,
	new_uri: Option<String>,
	new_price: Option<u64>
) -> ProgramResult {
	let account_iter = &mut accounts.iter();

	let nft_data_account_info = next_account_info(account_iter)?;
	let payer_info = next_account_info(account_iter)?;
	let nft_owner_address_info = next_account_info(account_iter)?;
	let nft_token_account_info = next_account_info(account_iter)?;
	let system_account_info = next_account_info(account_iter)?;

	let nft_data_seeds = &[
		PREFIX.as_bytes(),
		program_id.as_ref(),
		&[id]
	];

	let (nft_data_key, _nft_data_bump_seed) = Pubkey::find_program_address(nft_data_seeds, program_id);

	if *nft_data_account_info.key != nft_data_key {
		return Err(CustomError::InvalidNFTDataKey.into());
	}

	let mut nft_data = NFTData::from_acc_info(nft_data_account_info)?;
	let token_acc: Account = assert_initialized(&nft_token_account_info)?;

	if *nft_owner_address_info.key != token_acc.owner {
		return Err(CustomError::OwnerMismatch.into());
	}

	if nft_data.owner_address != token_acc.owner {
		return Err(CustomError::InvalidOwner.into());
	}

	invoke(
        &system_instruction::transfer(&payer_info.key, &nft_owner_address_info.key, nft_data.listed_price as u64),
        &[
            payer_info.clone(),
            nft_owner_address_info.clone(),
            system_account_info.clone(),
        ],
    )?;

	nft_data.name = match new_name {
		Some(new_name) => new_name,
		None => nft_data.name
	};

	nft_data.uri = match new_uri {
		Some(new_uri) => new_uri,
		None => nft_data.uri
	};

	nft_data.last_price = nft_data.listed_price;
    nft_data.listed_price = match new_price {
        Some(price) => {
            price
        }
        None => {
            nft_data.listed_price
        }
    };

	let mut array_of_zeroes = vec![];

	while array_of_zeroes.len() > MAX_NAME_LENGTH - nft_data.name.len() {
		array_of_zeroes.push(0u8);
	}

	nft_data.name = nft_data.name.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();

	let mut array_of_zeroes = vec![];

	while array_of_zeroes.len() > MAX_URI_LENGTH - nft_data.uri.len() {
		array_of_zeroes.push(0u8);
	}

	nft_data.uri = nft_data.uri.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();

	nft_data.serialize(&mut *nft_data_account_info.data.borrow_mut())?;
	msg!("nft Data Replaced!");

	Ok(())
}

pub fn assert_initialized<T: Pack + IsInitialized>(
	account_info: &AccountInfo,
) -> Result<T, ProgramError> {
	let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    	if !account.is_initialized() {
        	Err(CustomError::Uninitialized.into())
    	} else {
        	Ok(account)
    	}
}