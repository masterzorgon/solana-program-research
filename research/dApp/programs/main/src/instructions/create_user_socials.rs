use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod args;

use instructions::*;
use state::*;
use args::*;

pub fn create_user_socials(ctx: Context<CreateUserSocials>, args: SocialsArgs) -> Result<()> {
    let user_socials = &mut ctx.accounts.user_socials;

    user_socials.name = args.name;
    user_socials.discord = args.discord;
    user_socials.twitter = args.twitter;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUserSocials<'info> {
    #[account(
        mut,
        seeds = [
            UserSocials::PREFIX.as_ref(),
            user.key().as_ref()
        ],
        bump = user_socials.bump,
    )]
    pub user_socials: Account<'info, UserSocials>,
    pub user: Signer<'info>
}
