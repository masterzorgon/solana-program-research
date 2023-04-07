use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod introductions {
    use super::*;

    pub fn add_intro(
        ctx: Context<AddIntro>,
        args: IntroArgs
    ) -> Result<()> {
        msg!("Added Student Intro Account!");
        ctx.accounts.intro_account.new(args.name, args.content)
    }

    pub fn update_intro(
        ctx: Context<UpdateIntro>,
        args: IntroArgs
    ) -> Result<()> {
        msg!("Updated Student Intro Account!");
        ctx.accounts.intro_account.update(args.content)
    }

    pub fn close_intro(
        ctx: Context<CloseIntro>,
        args: IntroArgs
    ) -> Result<()> {
        msg!("Closed Student Intro Account!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(args: IntroArgs)]
pub struct CloseIntro<'info> {
    #[account(
        mut,
        seeds = [
            StudentIntroAccount::PREFIX, 
            args.name.as_bytes(), 
            initializer.key().as_ref()
        ],
        bump,
        close = initializer
    )]
    pub intro_account: Account<'info, StudentIntroAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(args: IntroArgs)]
pub struct UpdateIntro<'info> {
    #[account(
        mut,
        seeds = [
            StudentIntroAccount::PREFIX, 
            args.name.as_bytes(), 
            initializer.key().as_ref()
        ],
        bump,
        realloc = StudentIntroAccount::calc_len(&args.name, &args.content),
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub intro_account: Account<'info, StudentIntroAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)] // deserializes and validates specified accounts
#[instruction(args: IntroArgs)] // access instruction arguments
pub struct AddIntro<'info> {
    #[account( // specifies constraints on the account
        init,
        seeds = [
            StudentIntroAccount::PREFIX, 
            args.name.as_bytes(), 
            initializer.key().as_ref()
        ],
        bump,
        payer = initializer,
        space = StudentIntroAccount::calc_len(&args.name, &args.content)
    )] 
    pub intro_account: Account<'info, StudentIntroAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[account] // helps w (de)serialization, sets discriminator, sets owner as programID
pub struct StudentIntroAccount {
    pub name: String,
    pub content: String,
}

impl StudentIntroAccount {
    pub const PREFIX: &'static [u8] = b"INTRO_ACCOUNT";

    pub fn calc_len(name: &str, content: &str) -> usize {
        8 +                     // discriminator
        4 + name.len() +        // title
        4 + content.len()       // description
    }

    pub fn new(&mut self, name: String, content: String) -> Result<()> {
        self.name = name;
        self.content = content;
        Ok(())
    }

    pub fn update(&mut self, content: String) -> Result<()> {
        self.content = content;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct IntroArgs {
    pub name: String,
    pub content: String
}
