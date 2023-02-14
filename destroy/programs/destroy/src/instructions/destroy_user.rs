use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;
use crate::state::user::UserAccount;

#[derive(Accounts)]
pub struct DestroyUserContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [UserAccount::PREFIX.as_bytes(), user_account.signer.key().as_ref()],
        has_one = signer,
        bump = user_account.bump
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}

pub fn destroy_user(ctx: Context<DestroyUserContext>) -> Result<()> {
    let signer: &mut Signer = &mut ctx.accounts.signer;
    let user_account: &mut Account<UserAccount> = &mut ctx.accounts.user_account;

    // leftover rent is sent back to the user
    user_account.close(signer.to_account_info())?;

    Ok(())
}
