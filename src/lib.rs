use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::{AccountInfo, next_account_info},
    msg,
    pubkey::Pubkey,
}

use crate::instructions::CounterInstructions;

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct CounterAccount {
    counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _pubkey: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("This program has started processing!");

    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment => {
            counter_account.counter += 1;
        }
        CounterInstructions::Decrement => {
            counter_account.counter -= 1;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        } CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}