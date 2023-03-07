use anchor_lang::prelude::*;

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct SocialsArgs {
    pub name: String,
    pub discord: String,
    pub twitter: String,
}
