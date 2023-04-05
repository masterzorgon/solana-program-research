use anchor_lang::prelude::*;

use crate::state::{ business_unit::* };
use crate::errors::{ business_unit_errors::* };

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CreateBusinessUnitArgs {
    pub company_name: String,
    pub address: String,
    pub business_unit_name: String,
    pub supervisor: String,
    pub email: String,
    pub phone: String,
    pub routing_number: String
}

#[derive(Accounts)]
#[instruction(args: CreateBusinessUnitArgs)]
pub struct CreateBusinessUnit<'info> {
    #[account(
        init_if_needed,
        seeds = [
            BusinessUnit::PREFIX,
            b"_",
            &args.company_name.as_bytes(),
            b"_",
            &args.business_unit_name.as_bytes(),
            b"_",
            authority.key.as_ref()
        ],
        bump,
        payer = authority,
        space = BusinessUnit::calc_space(&args)
    )]
    pub business_unit: Account<'info, BusinessUnit>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>
}

pub fn create_business_unit(ctx: Context<CreateBusinessUnit>, args: CreateBusinessUnitArgs) -> Result<()> {
    let business_unit: &mut Account<BusinessUnit> = &mut ctx.accounts.business_unit;

    msg!("Fetching address and bump of business unit PDA.");
    let (pda, bump) = Pubkey::find_program_address(
        &[
            BusinessUnit::PREFIX,
            b"_",
            &args.company_name.as_bytes(),
            b"_",
            &args.business_unit_name.as_bytes(),
            b"_",
            ctx.accounts.authority.key.as_ref()
        ],
        ctx.program_id
    );

    msg!("Assigning data values to business unit PDA attributes.");
    require!(args.company_name.len() <= 30, BusinessUnitError::CompanyNameSizeInvalid);
    business_unit.company_name = args.company_name;

    require!(args.address.len() <= 40, BusinessUnitError::AddressSizeInvalid);
    business_unit.address = args.address;

    require!(args.business_unit_name.len() <= 30, BusinessUnitError::BusinessUnitNameSizeInvalid);
    business_unit.business_unit_name = args.business_unit_name;

    require!(args.supervisor.len() <= 30, BusinessUnitError::SupervisorNameSizeInvalid);
    business_unit.supervisor = args.supervisor;

    require!(args.email.len() <= 30, BusinessUnitError::EmailSizeInvalid);
    business_unit.email = args.email;

    require!(args.phone.len() == 10, BusinessUnitError::PhoneSizeInvalid);
    business_unit.phone = args.phone;

    require!(args.routing_number.len() == 10, BusinessUnitError::PhoneSizeInvalid);
    business_unit.routing_number = args.routing_number;

    business_unit.relationships = Vec::new();

    business_unit.created_at = ctx.accounts.clock.unix_timestamp;
    business_unit.bump = bump;
    business_unit.pda = pda;

    msg!("Business unit PDA created successfully!");
    Ok(())
}
