use anchor_lang::prelude::*;

mod instructions;
mod state;

pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pdas_accounts {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        instructions::add_movie_review(title, description, rating)
    }

    pub fn update_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8
    ) -> Result<()> {
        instructions::update_movie_review(title, description, rating)
    }

    // no logic needed. closing the account is handled by the runtime, invoked by DeleteMovieReview
    pub fn delete_movie_review(
        ctx: Context<DeleteMovieReview>,
        title: String
    ) -> Result<()> {
        instructions::delete_movie_review(title)
    }
}
