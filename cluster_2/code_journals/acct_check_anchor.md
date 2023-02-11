# Code Journal for the Anchor Cheking Accounts Program

First we import the `prelude` of the `anchor_lang` crate:

```rust
use anchor_lang::prelude::*;
```

We define the program’s ID using the `declare_id` macro:

```rust
declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");
```

Then we define a module containing al instruction habdlers defining all entries into our Solana program.

So, we aply the `#[program]` attribute to a pubic module, we resolve the parent module and then we define our instruction `check_accounts` wich recieves a context `Context<CheckingAccounts>`, in the body of the function we only return Ok(()):

```rust
#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {

        Ok(())
    }
}
```

Finaly we define context of our instruction.
We implement an `Accounts` deserializer on the `CheckingAccounts` struct.
Since `Accounts` holds refences, those references must have lifetimes so that the Rust Borrow Checker can verify their usage, in this case our lifestime is called `info`,
The `Signer` type validates that the accouns is a signer.
The `#[account(mut)]` attribute has the mut constrain, wich is needed if we will need to mutate this account.
The `/// CHECK:` comment allows us to skip doing checkings of that account here (we can do some custom checking in our instructions if is needed)
We will recieve 4 accounts: payer, account\_to\_create, account\_to\_change and system\_program

```rust
#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    system_program: Program<'info, System>,
}
```

### Additional Questions

***

##### What are the concepts?

* Crates (we import one using the use keyboard).
* Macros (We use declare\_id! and msg!).
* Lifetimes
* Traits be implement in our context

##### What is the organization?

We have a public module with a single instruction, wich recieves a context and
return a success value, this context is an struct, we implement some traits in
that struct to do the actual account checking.

##### What is the contract doing? What is the mechanism?

It simply do account checking and returns errors if something doesn't checks,
finally if everything is as expected return with success.

##### How could it be better? More efficient? Safer?

The code could be better if contains more test cases in the test folder.