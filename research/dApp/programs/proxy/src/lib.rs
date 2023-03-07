// use anchor_lang::prelude::*;
// use anchor_lang::context::CpiContext;

// declare_id!("Ci7X76k9sFimgZGSrm2F8DSkmKXVSUoqxQzg5nsH6ajc");

// #[program]
// pub mod proxy {
//     use super::*;

//     pub fn update_socials(ctx: Context<SetSocials>, args: SocialsArgs) -> Result<()> {
//         println!("Starting call to user socials");
//         main::cpi::create_user_socials(
//             ctx.accounts.set_socials(),
//             args.name,
//             args.twitter,
//             args.discord
//         )
//     }
// }

// #[derive(Accounts)]
// pub struct SetSocials<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(mut)]
//     pub user_socials: Account<'info, UserSocials>,
//     pub main: Program<'info, Main>,
// }

// impl<'info> SetSocials<'info> {
//     pub fn set_socials(&self) -> CpiContext<'_, '_, '_, 'info, CreateUserSocials<'info>> {
//         let cpi_program = self.main.to_account_info();
//         let cpi_accounts = CreateUserSocials {
//             user_socials: self.user_socials.to_account_info(),
//             user: self.user.to_account_info(),
//         };
//         CpiContext::new(cpi_program, cpi_accounts)
//     }
// }

// // import later
// #[derive(Debug, AnchorSerialize, AnchorDeserialize)]
// pub struct SocialsArgs {
//     pub name: String,
//     pub discord: String,
//     pub twitter: String,
// }

// #[account]
// pub struct UserSocials {
//     pub name: String,
//     pub discord: String,
//     pub twitter: String,
//     pub bump: u8
// }

// impl UserSocials {
//     pub const PREFIX: &'static [u8] = b"user_socials_";

//     pub const SPACE: usize = 8 + // discriminator
//         200 + // name
//         200 + // discord
//         200 + // twitter
//         1;    // bump
// }

// #[derive(Accounts)]
// pub struct CreateUserSocials<'info> {
//     #[account(
//         mut,
//         seeds = [
//             UserSocials::PREFIX.as_ref(),
//             user.key().as_ref()
//         ],
//         bump = user_socials.bump,
//     )]
//     pub user_socials: Account<'info, UserSocials>,
//     pub user: Signer<'info>
// }
