// no logic needed. closing the account is handled by the runtime, invoked by DeleteMovieReview
pub fn delete_movie_review(
    ctx: Context<DeleteMovieReview>,
    title: String
) -> Result<()> {
    msg!("Movie review for {} deleted", title);
    Ok(())
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
