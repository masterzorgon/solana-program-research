use anchor_lang::prelude::*;

#[account]
pub struct UserSocials {
    pub name: String,
    pub discord: String,
    pub twitter: String,
    pub bump: u8
}

impl UserSocials {
    pub const PREFIX: &'static [u8] = b"user_socials_";

    pub const SPACE: usize = 8 + // discriminator
        200 + // name
        200 + // discord
        200 + // twitter
        1;    // bump
}
