use anchor_lang::prelude::*;

mod state;
mod errors;
mod instructions;

use instructions::*;

declare_id!("A8aZpHiJhbHawHQ6g4mhaxPTXBGoc5bQecaRkro2hyGe");

#[program]
pub mod implementation {
    use super::*;

    pub fn create_business_unit(ctx: Context<CreateBusinessUnit>, args: CreateBusinessUnitArgs) -> Result<()> {
        create_business_unit::create_business_unit(ctx, args)
    }
}
