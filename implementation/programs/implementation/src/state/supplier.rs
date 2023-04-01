use anchor_lang::prelude::*;

use crate::args::{ supplier_args::* };

/*
    upon creation of supplier account, the supplier is defined 
    to have a transactional relationship with 1+ business units.

    for each transactional relationship, a master nft is minted. 
    the address of the master nft is stored with the supplier 
    account in the `relationships` attribute. 
    
    the master nft is then used to mint a child nft for each
    transaction that occurs between the supplier and the business
*/

#[account]
pub struct Supplier {
    pub signer: Pubkey,
    pub bump: u8,
    pub identifier: Pubkey,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: String,
    pub relationships: Vec<Pubkey>,
    pub total_transactions: u64,
}

impl Supplier {
    pub const PREFIX: &'static [u8] = b"SUPPLIER";

    pub fn calc_space(args: &CreateSupplierArgs) -> usize {
        32 + // signer
        8 + // discriminator
        1 + // bump
        32 + // identifier
        args.name.len() + // name
        args.address.len() + // address
        args.phone.len() + // phone
        args.email.len() + // email
        args.routing_number.len() + // routing_number (base64-bash encoded string)
        (6 * 32) + // relationships
        8 // total_transactions
    }
}
