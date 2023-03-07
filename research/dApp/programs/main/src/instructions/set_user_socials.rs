use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod args;

use instructions::*;
use state::*;
use args::*;

pub fn set_user_socials(ctx: Context<UpdateUserSocials>) -> Result<()> {
    let user_socials = &mut ctx.accounts.user_socials;

    user_socials.name = String::from("");
    user_socials.discord = String::from("");
    user_socials.twitter = String::from("");;
    user_socials.bump = *ctx.bumps.get("user_socials_").unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateUserSocials<'info> {
    #[account(
        init,
        payer = user,
        seeds = [
            UserSocials::PREFIX.as_ref(),
            user.key().as_ref()
        ],
        bump,
        space = UserSocials::SPACE
    )]
    pub user_socials: Account<'info, UserSocials>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}
