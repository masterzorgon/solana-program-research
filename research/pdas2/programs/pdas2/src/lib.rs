use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pdas2 {
    use super::*;

    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        
        require!(name.as_bytes().len() <= 200, UserStatsError::NameTooLong);
        
        user_stats.level = 0u16;
        user_stats.name = name;
        user_stats.bump = *ctx.bumps.get("user_stats").unwrap();

        Ok(())
    }

    pub fn change_user_name(ctx: Context<ChangeUserName>, name: String) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;

        require!(name.as_bytes().len() <= 200, UserStatsError::NameTooLong);

        user_stats.name = name;

        Ok(())
    }
}

#[error_code]
pub enum UserStatsError {
    #[msg("Name is too long")]
    NameTooLong,
}

#[account]
pub struct UserStats {
    level: u16,
    name: String,
    bump: u8
}

impl UserStats {
    pub const PREFIX: &'static [u8] = b"user_stats_";

    pub const SPACE: usize = 8 + // bump
        2 + // level
        200 + // name
        1; // bump
}

#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(
        init,
        payer = user,
        space = UserStats::SPACE,
        seeds = [
            UserStats::PREFIX.as_ref(),
            user.key().as_ref()
        ],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

// validation struct
#[derive(Accounts)]
pub struct ChangeUserName<'info> {
    #[account(
        mut,
        seeds = [
            UserStats::PREFIX.as_ref(),
            user.key().as_ref()
        ],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserStats>,
    pub user: Signer<'info>
}
