use anchor_lang::prelude::*;

mod state;
mod errors;
mod instructions;

use instructions::*;

declare_id!("A8aZpHiJhbHawHQ6g4mhaxPTXBGoc5bQecaRkro2hyGe");

#[program]
pub mod implementation {
    use super::*;

    pub fn create_supplier(ctx: Context<CreateSupplier>, args: SupplierArgs) -> Result<()> {
        create_supplier::create_supplier(ctx, args)
    }
}
