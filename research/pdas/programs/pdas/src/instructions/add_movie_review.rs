use anchor_lang::prelude::*;

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
