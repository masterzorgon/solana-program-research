use anchor_lang::prelude::*;

// all fields are mandatory
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateSupplierArgs {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: String,
}

// all fields are optional
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdateSupplierArgs {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub routing_number: Option<String>,
}
