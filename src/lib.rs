use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

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
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use solana_program_test::tokio::process;
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let increment_instruction_data: Vec<u8> = vec![0];
        let decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 1);

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 0);

        let update_value = 45u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();
        
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 45);

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 0);

    }
}
