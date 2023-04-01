use anchor_lang::prelude::*;

mod args;
mod state;
mod errors;
mod instructions;

use args::supplier_args::*;
use instructions::*;

declare_id!("A8aZpHiJhbHawHQ6g4mhaxPTXBGoc5bQecaRkro2hyGe");

#[program]
pub mod implementation {
    use super::*;

    pub fn create_supplier(ctx: Context<CreateSupplier>, args: SupplierArgs, relationships: Vec<Relationship>) -> Result<()> {
        create_supplier::create_supplier(ctx, args, relationships)
    }

    pub fn update_supplier(ctx: Context<UpdateSupplier>, args: SupplierArgs) -> Result<()> {
        update_supplier::update_supplier(ctx, args)
    }
}
