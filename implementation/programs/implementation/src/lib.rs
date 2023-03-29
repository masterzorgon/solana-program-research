use anchor_lang::prelude::*;

mod state;
mod instructions;

use instructions::*;

declare_id!("A8aZpHiJhbHawHQ6g4mhaxPTXBGoc5bQecaRkro2hyGe");

#[program]
pub mod implementation {
    use super::*;

    pub fn create_supplier(ctx: Context<CreateSupplier>, args: CreateSupplierArgs) -> Result<()> {
        create_supplier::create_supplier(ctx, args)
    }
}
