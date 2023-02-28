use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked,
};
use std::convert::TryInto;
pub mod instruction;
pub mod state;
use instruction::zilchInstruction;
use state::zilchAccountState;
use borsh::BorshSerialize;

entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  let instruction = zilchInstruction::unpack(instruction_data)?;
  match instruction {
    zilchInstruction::Addzilchprogram { program_hash, outputs, proof_account } => {
      add_zilch_program(program_id, accounts, program_hash, outputs, proof_account)
    }
  }
}

pub fn add_zilch_program(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  program_hash: String,
  outputs: u8,
  proof_account: String
) -> ProgramResult {
  msg!("Adding zilch program...");
  msg!("program_hash: {}", program_hash);
  msg!("outputs: {}", outputs);
  msg!("proof_account: {}", proof_account);

// Get Account iterator
  let account_info_iter = &mut accounts.iter();

// Get accounts
  let initializer = next_account_info(account_info_iter)?;
  let pda_account = next_account_info(account_info_iter)?;
  let system_program = next_account_info(account_info_iter)?;

// Derive PDA and check that it matches client
  let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), program_hash.as_bytes().as_ref(),], program_id);

// Calculate account size required
  let account_len: usize = 1 + 1 + (4 + program_hash.len()) + (4 + proof_account.len());

// Calculate rent required
  let rent = Rent::get()?;
  let rent_lamports = rent.minimum_balance(account_len);

// Create the account
  invoke_signed(
    &system_instruction::create_account(
      initializer.key,
      pda_account.key,
      rent_lamports,
      account_len.try_into().unwrap(),
      program_id,
    ),
    &[initializer.clone(), pda_account.clone(), system_program.clone()],
    &[&[initializer.key.as_ref(), program_hash.as_bytes().as_ref(), &[bump_seed]]],
  )?;

  msg!("PDA created: {}", pda);

  msg!("unpacking state account");
  let mut account_data = try_from_slice_unchecked::<zilchAccountState>(&pda_account.data.borrow()).unwrap();
  msg!("borrowed account data");

  account_data.program_hash = program_hash;
  account_data.outputs = outputs;
  account_data.proof_account = proof_account;
  account_data.is_initialized = true;

  msg!("serializing account");
  account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
  msg!("state account serialized");

  Ok(())
}
