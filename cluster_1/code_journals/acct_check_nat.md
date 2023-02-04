# Code Journal for the Native Cheking Accounts Program

First we import everything we need from the `solana_program` crate:

```rust
use solana_program::{
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};
```

Then, we declare the program entry point using the `entrypoint` macro:

```rust
entrypoint!(process_instruction);
```

After that, We declare a function called `process_instrucction` (which is the entry point of the program, as we establish before using the `entrypoint` macro, this function will receive 3 parameters ( `program_id` as a reference to a `Pubkey`, `accounts` as a reference to an array of `AccountInfo` values, and `_instruction_data` as a reference to an array of unsigned 8 bytes values. The underscore before the last parameter indicates that we don't will use that one ) and this function will return a `ProgramResult`. So, the signature of the function is:

```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
```

Inside this function the first thing we do is some basic checking, so in case the program ID is not the one from our program or if the amount of accounts in the accounts array is not the right one we throw some helpful errors and finish the program:

```rust
// You can verify the program ID from the instruction is in fact 
    //      the program ID of your program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };
    
    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you 
    //      try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };
```

Then we assign each account of the array to an individual variable:

```rust
    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
```

We check if the account we want to create is not initialized simply by checking if has 0 Lamports if not we throw an error and finish the program

```rust
 // You can make sure an account has NOT been initialized.
    
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)
```

We check if the account we want change is already initialized by looking if has more than 0 lamports (an account needs to have some SOL in order to pay rent, if has the amount needed to pay for 2 year becomes rent exempt)

```rust
    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };
```

We check if our program is the owner of the account (only the owner of an account can change his data, the only thing that a non owner can do is increment the amount of the account, and a program is the owner of the PDAs that it creates)

```rust
    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };
```

This is super important: you need to check that the system program is actually the system program if not a hacker can pass his own program that emulates the system program to authorize a behavior that is not the intended one, and, for example stole all you money:

```rust

   // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };
```

Finally if everything is ok we return a success value and finish our function:

```rust
    Ok(())
}
```