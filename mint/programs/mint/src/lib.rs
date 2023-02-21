use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mint {
    use super::*;

    pub fn create_mint(
        ctx: Context<CreateTokenMint>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
        mint_authority_pda_bump: u8
    ) -> Result<()> {
        create_token_mint::create_token_mint(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
            mint_authority_pda_bump
        )
    }
}
