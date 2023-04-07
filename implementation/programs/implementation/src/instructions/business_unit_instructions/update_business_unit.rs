use anchor_lang::prelude::*;

use crate::state::{ business_unit::* };
use crate::errors::{ business_unit_errors::* };

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct UpdateBusinessUnitArgs {
    pub company_name: Option<String>,
    pub address: Option<String>,
    pub business_unit_name: Option<String>,
    pub supervisor: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub routing_number: Option<String>,
}

#[derive(Accounts)]
#[instruction(args: UpdateBusinessUnitArgs)]
pub struct UpdateBusinessUnit<'info> {
    #[account(
        mut,
        seeds = [
            BusinessUnit::PREFIX,
            b"_",
            &args.company_name.unwrap().as_bytes(),
            b"_",
            &args.business_unit_name.unwrap().as_bytes(),
            b"_",
            authority.key.as_ref()
        ],
        bump = business_unit.bump,
    )]
    pub business_unit: Account<'info, BusinessUnit>,

    pub authority: Signer<'info>,
}

pub fn update_business_unit(ctx: Context<UpdateBusinessUnit>, args: UpdateBusinessUnitArgs) -> Result<()> {
    let business_unit: &mut Account<BusinessUnit> = &mut ctx.accounts.business_unit;

    msg!("Updating data values to business unit PDA attributes.");
    
    if let Some(company_name) = args.company_name {
        require!(company_name.len() <= 30, BusinessUnitError::CompanyNameSizeInvalid);
        business_unit.company_name = company_name;
    }

    if let Some(address) = args.address {
        require!(address.len() <= 40, BusinessUnitError::AddressSizeInvalid);
        business_unit.address = address;
    }

    if let Some(business_unit_name) = args.business_unit_name {
        require!(business_unit_name.len() <= 30, BusinessUnitError::BusinessUnitNameSizeInvalid);
        business_unit.business_unit_name = business_unit_name;
    }

    if let Some(supervisor) = args.supervisor {
        require!(supervisor.len() <= 30, BusinessUnitError::SupervisorNameSizeInvalid);
        business_unit.supervisor = supervisor;
    }

    if let Some(email) = args.email {
        require!(email.len() <= 30, BusinessUnitError::EmailSizeInvalid);
        business_unit.email = email;
    }

    if let Some(phone) = args.phone {
        require!(phone.len() == 10, BusinessUnitError::PhoneSizeInvalid);
        business_unit.phone = phone;
    }

    if let Some(routing_number) = args.routing_number {
        require!(routing_number.len() == 10, BusinessUnitError::PhoneSizeInvalid);
        business_unit.routing_number = routing_number;
    }

    msg!("Business unit PDA updated successfully!");

    Ok(())
}
