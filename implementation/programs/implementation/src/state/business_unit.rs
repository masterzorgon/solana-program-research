use anchor_lang::prelude::*;

use crate::instructions::{ create_business_unit::* };

#[account]
pub struct BusinessUnit {
    // input values, dynamically sized
    pub company_name: String,
    pub address: String,
    pub business_unit_name: String,
    pub supervisor: String,
    pub email: String,
    pub phone: String,
    pub routing_number: String, // routing_number (base64-bash encoded string)

    // dynamically sized; updated over time
    pub relationships: Vec<Pubkey>,

    // auto generated
    pub created_at: i64
    pub bump: u8,
    pub address: Pubkey,
}

impl BusinessUnit {
    pub const PREFIX: &'static [u8] = b"business_unit";

    pub fn calc_space(args: &CreateBusinessUnitArgs) -> usize {
        args.company_name.len() + // company_name
        args.address.len() +  // address
        args.business_unit_name.len() + // business_unit_name
        args.supervisor.len() + // supervisor
        args.email.len() + // email
        args.phone.len() + // phone (max: min: 10)
        args.routing_number.len() + // routing_number (max: 10)

        (6 * 32) + // (number of relationships * pubkey space) <- initial size; PDA size updated over time using `realloc`
        
        8 + // discriminator
        8 + // created_at
        1 + // bump
        32; // address
    }
}
