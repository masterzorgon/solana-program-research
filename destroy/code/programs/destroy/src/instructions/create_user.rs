use anchor_lang::prelude::*;
use crate::state::user::UserAccount;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserArgs {
    pub name: String
}

#[derive(Accounts)]
#[instruction(args: CreateUserArgs)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [UserAccount::PREFIX.as_bytes(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = UserAccount::SIZE
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}

pub fn create_user(
    ctx: Context<CreateUserContext>,
    args: CreateUserArgs
) -> Result<()> {
    let signer: &Signer = &ctx.accounts.signer;
    let user_account: &mut Account<UserAccount> = &mut ctx.accounts.user_account;

    user_account.bump = *ctx.bumps.get("user_account").unwrap();
    user_account.signer = signer.key();
    user_account.name = args.name;

    Ok(())
}
