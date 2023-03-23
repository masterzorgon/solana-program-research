use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cpipda {
    use super::*;

    pub fn set_user_socials(ctx: Context<UpdateUserSocials>) -> Result<()> {
        let user_socials = &mut ctx.accounts.user_socials;
        user_socials.name = String::from("");
        user_socials.discord = String::from("");
        user_socials.twitter = String::from("");
        user_socials.bump = *ctx.bumps.get(UserInfo::PREFIX).unwrap();
        Ok(())
    }

    pub fn create_user_socials(ctx: Context<CreateUserSocials>, args: CreateUserSocialsArgs) -> Result<()> {
        let user_socials = &mut ctx.accounts.user_socials;
        user_socials.name = args.name;
        user_socials.discord = args.discord;
        user_socials.twitter = args.twitter;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct CreateUserSocialsArgs {
    pub name: String,
    pub discord: String,
    pub twitter: String
}

#[derive(Accounts)]
pub struct CreateUserSocials<'info> {
    #[account(
        mut,
        seeds = [
            UserInfo::PREFIX.as_ref(),
            user.key().as_ref()
        ],
        bump = user_socials.bump
    )]
    pub user_socials: Account<'iffnfo, UserInfo>,

    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpdateUserSocials<'info> {
    #[account(
        init,
        payer = user,
        space = UserInfo::SPACE,
        seeds = [
            UserInfo::PREFIX.as_ref(), 
            user.key().as_ref()
        ],
        bump,
    )]
    pub user_socials: Account<'info, UserInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserInfo {
    pub name: String,
    pub discord: String,
    pub twitter: String,
    pub bump: u8
}

impl UserInfo {
    pub const PREFIX: &'static str = "user_socials_";

    pub const SPACE: usize = 8 + // discriminator
        200 + // name
        200 + // discord
        200 + // twitter
        1; // bump
}
