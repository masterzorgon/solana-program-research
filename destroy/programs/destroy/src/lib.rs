use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("Gxe7wba1Csse9G1C8Y6jg9J1Wv8ZF26s5ZVfFrDw1Stb");

#[program]
pub mod destroy_program {
    use super::*;

    pub fn create_user_account(
        ctx: Context<CreateUserContext>,
        args: CreateUserArgs
    ) -> Result<()> {
        create_user::create_user(ctx, args)
    }

    pub fn destroy_user_account(
        ctx: Context<DestroyUserContext>
    ) -> Result<()> {
        destroy_user::destroy_user(ctx)
    }
}
