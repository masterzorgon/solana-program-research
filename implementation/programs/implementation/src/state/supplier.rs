use anchor_lang::prelude::*;

use crate::args::{ supplier_args::* };

/*
    upon creation of supplier account, the supplier is defined to have a transactional relationship
    with 1+ business units.

    for each transactional relationship, a master nft is minted. the address of the master nft is
    stored with the supplier account. the master nft is then used to mint a child nft for each
*/

#[account]
pub struct Supplier {
    pub bump: u8,
    pub identifier: Pubkey,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: u32,
    pub relationships: Vec<Relationship>,
    pub total_transactions: u64,
}

impl Supplier {
    pub const PREFIX: &'static [u8] = b"SUPPLIER";

    pub fn calc_space(args: &SupplierArgs) -> usize {
        8 + // discriminator
        1 + // bump
        32 + // identifier
        args.name.len() + // name
        args.address.len() + // address
        args.phone.len() + // phone
        args.email.len() + // email
        4 + // routing_number
        (
            args.relationships.len() * (
                40 + // supplier
                40 + // business_unit
                32 // master_edition
            )
        ) + // relationships
        8 // total_transactions
    }
}
