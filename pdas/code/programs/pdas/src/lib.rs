use anchor_lang::prelude::*;

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
        msg!("Movie Review Account Created!");
        msg!("Title: {}", title);
        msg!("description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8
    ) -> Result<()> {
        msg!("Movie review account space reallocated");
        msg!("title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    // no logic needed. closing the account is handled by the runtime, invoked by DeleteMovieReview
    pub fn delete_movie_review(
        ctx: Context<DeleteMovieReview>,
        title: String
    ) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer // rent refunded to initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = MovieAccountState::calc_len(&title, &description), // recalculate space (for rent) since description might be diff length
        realloc::payer = initializer, // lamports refunded/required will be paid to/by initializer
        realloc::zero = true // movie_review account may be updated multiple times, shrinking/expanding the allocated account space
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)] // deserializes and validates specified accounts
#[instruction(title: String, description: String)] // access instruction arguments
pub struct AddMovieReview<'info> {
    #[account( // specifies constraints on the account
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = MovieAccountState::calc_len(&title, &description)
    )] 
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[account] // helps w (de)serialization, sets discriminator, sets owner as programID
pub struct MovieAccountState {
    pub reviewer: Pubkey,       // 32
    pub rating: u8,             // 1
    pub title: String,          // 4 + len()
    pub description: String     // 4 + len()
}

impl MovieAccountState {
    pub fn calc_len(title: &str, description: &str) -> usize {
        8 +                     // discriminator
        32 +                    // reviewer
        1 +                     // rating
        4 + title.len() +       // title
        4 + description.len()   // description
    }
}
