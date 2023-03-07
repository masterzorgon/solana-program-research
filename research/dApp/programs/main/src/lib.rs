use anchor_lang::prelude::*;

declare_id!("GwX7HeaaspNTYa6XocC9gDyPRD6jWKujPMarrJE6iNP4");

// pub mod instructions;
// pub mod state;
// pub mod args;

// use instructions::*;
// use state::*;
// use args::*;

#[program]
pub mod main {
    use super::*;

    pub fn set_user_socials(ctx: Context<UpdateUserSocials>) -> Result<()> {
        instructions::set_user_socials(ctx)
    }

    pub fn create_user_socials(ctx: Context<CreateUserSocials>, args: SocialsArgs) -> Result<()> {
        instructions::create_user_socials(ctx, args)
    }
}
