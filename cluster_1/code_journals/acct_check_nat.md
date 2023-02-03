# Code Journal for the Native Cheking Accounts Program

First we import everything we will need from the `solana_program` crate:

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

After that, We declare a function called `process_instrucction` (wich is the entry point of the program, as we stablish before using the `entrypoint` macro, this function will recive 3 parameters ( `program_id` as a reference to a `Pubkey`, `accounts` as a reference to an array of `AccountInfo` values, and `_instruction_data` as a reference to an array of unsigned 8 bytes values. The underscore before the last parameter indicate that we don't will use that one ) and this function will return a `ProgramResult`. So, the signature of the function is:

```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
```

Inside this function the fist thing we do is some basic cheking, so in case the program ID is not the one from our program or if the amount of accounts in the accounts array is not the rigth one we trow some helpfull errors and finish the program:

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
