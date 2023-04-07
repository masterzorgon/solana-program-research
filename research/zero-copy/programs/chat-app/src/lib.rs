use anchor_lang::prelude::*;
use anchor_lang::accounts::loader::Loader;

declare_id!("8PKu3LwmnEvEesuMDee31MAanxrAsxAVS7eCMaku9sar");

#[program]
pub mod chat_app {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
        ctx.accounts.user.name = name;
        ctx.accounts.user.authority = *ctx.accounts.authority.key;
        ctx.accounts.user.bump = *ctx.bumps.get("user").unwrap();
        Ok(())
    }

    pub fn create_chat_room(ctx: Context<CreateChatRoom>, name: String) -> Result<()> {
        let given_name = name.as_bytes();
        let mut name = [0u8; 280];
        name[..given_name.len()].copy_from_slice(given_name);
        let mut chat = ctx.accounts.chat_room.load_init()?;
        chat.name = name;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateUser<'info> {
    #[account(
        init,
        seeds = [authority.key().as_ref(), User::PREFIX.as_bytes()],
        bump,
        payer = authority,
        space = User::SPACE
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateChatRoom<'info> {
    #[account(zero)]
    chat_room: Loader<'info, ChatRoom>
}

#[account(zero_copy)]
pub struct ChatRoom {
    head: u64,
    tail: u64,
    name: [u8; 280],
    messages: [Message; 33607]
}

impl ChatRoom {
    fn append(&mut self, message: Message) {
        self.messages[ChatRoom::index_of(self.head)] = message;
        if ChatRoom::index_of(self.head + 1) == ChatRoom::index_of(self.tail) {
            self.tail
                .checked_add(1)
                .unwrap();
        }
    }

    fn index_of(counter: u64) -> usize {
        std::convert::TryInto::try_into( // a generic method used for safe type conversions between number types while avoiding overflow issues
            counter % 33607 // this operation helps in constraining the input value to fit within a specific range.
        ).unwrap()
    }
}

#[zero_copy]
pub struct Message {
    pub from: Pubkey,
    pub data: [u8; 280]
}

#[account]
pub struct User {
    name: String,
    authority: Pubkey,
    bump: u8
}

impl User {
    pub const PREFIX: &'static str = "USER";

    pub const SPACE: usize = 8 + // discriminator
        32 + // authority
        1 + // bump 
        200; // name
}

#[error_code]
pub enum ErrorCode {
    Unknown
}
