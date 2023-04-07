use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token;
use mpl_token_metadata::instruction as mpl_instruction;

pub fn create_token_mint(
    ctx: Context<CreateTokenMint>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
    mint_authority_pda_bump: u8
) -> Result<()> {
    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account.key());

    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(), // program id
            ctx.accounts.metadata_account.key(), // metadata account
            ctx.accounts.mint_account.key(), // mint account
            ctx.accounts.mint_authority.key(), // mint authority
            ctx.accounts.payer.key(), // payer
            ctx.accounts.mint_account.key(), // update authority
            metadata_title, // name
            metadata_symbol, // symbol
            metadata_uri, // uri
            None, // creators
            0, // seller fee basis points
            true, // update authority is signer
            false, // is mutable
            None, // collection
            None // uses
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_into()
        ],
        &[
            &[
                b"mint_authority_",
                ctx.accounts.mint_account.key().as_ref(),
                &[mint_authority_pda_bump]
            ]
        ]
    )?;

    msg!("Token mint created successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMint<'info> {
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>, // creating this with metaplex

    #[account(
        init,
        payer = payer,
        space = 8 + 32,
        seeds = [b"mint_authority_", mint_account.key().as_ref()],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,

    pub token_metadata_program: UncheckedAccount<'info> // metaplex will check this
}

#[account]
pub struct MintAuthorityPda {}
