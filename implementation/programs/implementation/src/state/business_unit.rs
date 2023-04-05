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
    // input values, dynamically sized
    pub company_name: String,
    pub address: String,
    pub business_unit_name: String,
    pub supervisor: String,
    pub email: String,
    pub phone: String,
    pub routing_number: String, // routing_number (base64-bash encoded string)
    pub business_unit_type: BusinessUnitType,

    // dynamically sized; updated over time
    pub relationships: Vec<Pubkey>,
    pub relationship_function: RelationshipFunction,

    // auto generated
    pub created_at: i64,
    pub bump: u8,
    pub pda: Pubkey,
}

impl BusinessUnit {
    pub const PREFIX: &'static [u8] = b"business_unit";

    pub fn calc_space(args: &CreateBusinessUnitArgs) -> usize {
        let space: usize = args.company_name.len() +    // company_name
        args.address.len() +                            // address
        args.business_unit_name.len() +                 // business_unit_name
        args.supervisor.len() +                         // supervisor
        args.email.len() +                              // email
        args.phone.len() +                              // phone (max: min: 10)
        args.routing_number.len() +                     // routing_number (max: 10)
        1 +                                             // business_unit_type
        (6 * 32) +                                      // (number of relationships * pubkey space) <- initial size; PDA size updated over time using `realloc`
        1 +                                             // relationship_function
        8 +                                             // discriminator
        8 +                                             // created_at
        1 +                                             // bump
        32;                                             // address

        return space;
    }
}
