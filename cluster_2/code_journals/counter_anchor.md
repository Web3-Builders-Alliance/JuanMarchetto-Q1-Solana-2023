# Code Journal for the Anchor Counter Program

We import the `prelude` of the `anchor_lang` crate and we define the
program’s ID using the `declare_id` macro:

```rust
use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
```

Then we define our `counter_anchor` module containing the two instructions of our program 

```rust
#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }
}
```

`initialize_counter` only recieve a context, and handle the account initialization
`increment` only recieve a context, and handle the value change

we define our data account, wich only contains an u64

```rust
#[account]
pub struct Counter {
    count: u64,
}
```

And we define the contexts with de accounts needed for each ix,
in the fist one we do the account initialization
`#[account(init, space=8+8, payer=payer)]`
the 8+8 space comes from the space needed for the u64 and the space needed for the discriminator

```rust
#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, space=8+8, payer=payer)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
```

### Additional Questions

***

##### What are the concepts?

* Crates (we import one using the `use` keyboard).
* Macros (We use `declare_id!`).
* Lifetimes
* Traits we implement in our context

##### What is the organization?

We have a public module with a two instructions,
One colled `initialize_counter`wich recieves a context and
return a success value,
And other called `increment` wich recieves a context 
change the data inside the `Counter` account incrementint the `count`

And finally we have the definition of the contexts

##### What is the contract doing? What is the mechanism?

Initialize a counter account and increment it each time the incremet ix is called.

##### How could it be better? More efficient? Safer?

The code could be better if we do:
```rust
ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1)
```
insted of:
```rust
ctx.accounts.counter.count += 1;
```