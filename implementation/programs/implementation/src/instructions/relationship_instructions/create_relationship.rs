// use {
//     anchor_lang::{
//         prelude::*,
//         solana_program::program::invoke,
//         system_program
//     },
//     anchor_spl::{
//         associated_token,
//         token
//     },
//     mpl_token_metadata::{
//         ID as TOKEN_METADATA_ID,
//         instruction as token_instruction
//     },
// };

// #[derive(Accounts)]
// pub struct CreateRelationship<'info> {
//     /// CHECK: We're about to create this with Metaplex
//     #[account(mut)]
//     pub metadata: UncheckedAccount<'info>,

//     // CHECK: We're about to create this with Metaplex
//     #[account(mut)]
//     pub master_edition: UncheckedAccount<'info>,

//     /// CHECK: We're about to create this with Anchor
//     #[account(mut)]
//     pub token_account: UncheckedAccount<'info>,

//     /// CHECK: Metaplex will check this
//     pub token_metadata_program: UncheckedAccount<'info>,

//     #[account(mut)]
//     pub mint: Signer<'info>,

//     #[account(mut)]
//     pub mint_authority: Signer<'info>,

//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, token::Token>,
//     pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
// }

// pub fn create_relationship(ctx: Context<CreateRelationship>) -> Result<()> {
//     msg!("Creating mint account...");
//     msg!("Mint: {}", &ctx.accounts.mint.key());

//     system_program::create_account(
//         CpiContext::new(
//             &ctx.accounts.token_program.to_account_info(),   // context
//             system_program::CreateAccount {                 // instruction
//                 from: ctx.accounts.mint_authority.to_account_info(), 
//                 to: ctx.accounts.mint.to_account_info(),
//             }
//         ),
//         10_000_000,
//         82,
//         ctx.accounts.token_program.key()
//     )?;

//     msg!("Initializing mint account...");
//     msg!("Mint: {}", &ctx.accounts.mint.key());

//     msg!("Creating token account...");
//     msg!("Token address: {}", &ctx.accounts.token_account.key());

//     msg!("Minting token to token account...");
//     msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
//     msg!("Token address: {}", &ctx.accounts.token_account.key());

//     msg!("Creating metadata account...");
//     msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info(

//     ).key());

//     msg!("Creating master edition account...");
//     msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());

//     msg!("Token mint process completed successfully!");

//     Ok(())
// }
