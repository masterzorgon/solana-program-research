// ALTERNATIVE METHOD

// pub fn delete_movie_review(ctx: Context<DestroyUserContext>) -> Result<()> {
//     let movie_review: &mut Signer = &mut ctx.accounts.movie_review;
//     let initializer: &mut Account<UserAccount> = &mut ctx.accounts.initializer;

//     // leftover rent is sent back to the user
//     movie_review.close(initializer.to_account_info())?;

//     Ok(())
// }

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
