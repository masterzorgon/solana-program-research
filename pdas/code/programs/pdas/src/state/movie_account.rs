use anchor_lang::prelude::*;

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
