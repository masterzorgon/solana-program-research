use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Relationship {
    pub supplier: String,
    pub business_unit: String,
    pub master_edition: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SupplierArgs {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: u32,
    pub relationships: Vec<Relationship>,
}
