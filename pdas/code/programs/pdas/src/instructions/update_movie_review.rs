use anchor_lang::prelude::*;

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
