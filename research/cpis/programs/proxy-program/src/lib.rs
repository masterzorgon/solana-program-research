use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("fcB1A9en1FaChCDrUY41nNkp8c7KWTCjkRGVYgS6SCV");

#[program]
pub mod proxy_program {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        // in the "puppet" program, we're going to perform a cross program invocation on the `set_data` function
        puppet::cpi::set_data(ctx.accounts.set_data_ctx(), data)
    }
}
 
#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet> // define the "puppet" program
}

// move cpi setup into impl block to not distract from the business logic
impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info()
        };
        // create a new cross program invocation call
            // cpi_program = the program we're going to invoke - relevant to the solana runtime
            // cpi_accounts = the accounts we're going to pass to the program - relevant to the program instruction we're invoking
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
