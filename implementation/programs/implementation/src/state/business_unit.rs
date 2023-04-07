use anchor_lang::prelude::*;

use crate::instructions::{ 
    business_unit_instructions::*
};

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum BusinessUnitType {
    Supplier,
    Contractor,
    Manufacturer,
    Distributor,
    Retailer,
    Purchaser,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum RelationshipFunction {
    Buyer,
    Seller,
}

#[account]
pub struct BusinessUnit {
    // input values (dynamically sized)
    pub company_name: String,
    pub address: String,
    pub business_unit_name: String,
    pub supervisor: String,
    pub email: String,
    pub phone: String,

    // input values (fixed size)
    pub routing_number: String, // routing_number (base64-bash encoded string)
    pub business_unit_type: BusinessUnitType,
    pub relationship_function: RelationshipFunction,

    // dynamically sized; updated over time
    pub relationships: Vec<Pubkey>,

    // auto generated / assigned
    pub created_at: i64,
    pub bump: u8,
    pub pda: Pubkey,
    pub authority: Pubkey,
}

impl BusinessUnit {
    pub const PREFIX: &'static [u8] = b"business_unit";

    pub fn calc_space(args: &CreateBusinessUnitArgs) -> usize {
        let space: usize = 4 + args.company_name.len() +    // company_name
        4 + args.address.len() +                            // address
        4 + args.business_unit_name.len() +                 // business_unit_name
        4 + args.supervisor.len() +                         // supervisor
        4 + args.email.len() +                              // email
        4 + args.phone.len() +                              // phone (max: min: 10)
        4 + args.routing_number.len() +                     // routing_number (max: 10)
        1 +                                                 // business_unit_type
        4 + (6 as usize) * std::mem::size_of::<Pubkey>();   // (number of relationships * pubkey space) <- initial size; PDA size updated over time using `realloc`
        1 +                                                 // relationship_function
        8 +                                                 // discriminator
        8 +                                                 // created_at
        1 +                                                 // bump
        32 +                                                // pda
        32;                                                 // authority

        return space;
    }
}
