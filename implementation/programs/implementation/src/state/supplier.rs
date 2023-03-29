use anchor_lang::prelude::*;

#[account]
pub struct Supplier {
    pub bump: u8,
    pub identifier: Pubkey,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: u64,
    pub invoices: Vec<Pubkey>,
    pub total_transactions: u64,
}

impl Supplier {
    pub const PREFIX: &'static [u8] = b"SUPPLIER";

    pub fn calc_space(args: &CreateSupplierArgs) -> usize {
        8 + // discriminator
        1 + // bump
        32 + // identifier
        args.name.len() + // name
        args.address.len() + // address
        args.phone.len() + // phone
        args.email.len() + // email
        8 + // routing_number
        1_000 + // invoices
        8 // total_transactions
    }
}
