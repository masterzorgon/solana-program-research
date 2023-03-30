use anchor_lang::prelude::*;

use crate::args::{ supplier_args::SupplierArgs };
use crate::state::{ supplier::Supplier };
use crate::errors::{ supplier_errors::SupplierError };

#[derive(Accounts)]
#[instruction(args: SupplierArgs)]
pub struct UpdateSupplier<'info> {
    #[account(
        mut,
        seeds = [
            Supplier::PREFIX.as_ref(),
            b"_",
            &args.name.as_bytes(),
            b"_",
            signer.key.as_ref()
        ],
        bump = supplier.bump,
    )]
    pub supplier: Account<'info, Supplier>,
    
    pub signer: Signer<'info>,
}

pub fn update_supplier(ctx: Context<UpdateSupplier>, args: SupplierArgs) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;

    // make sure name is not too long
    require!(args.name.len() <= 40, SupplierError::SupplierNameTooLong);
    supplier.name = args.name;

    // make sure address is not too long
    require!(args.address.len() <= 40, SupplierError::SupplierAddressTooLong);
    supplier.address = args.address;

    // make sure phone is not too long
    require!(args.phone.len() <= 40 && args.phone.len() >= 10, SupplierError::SupplierPhoneTooLong);
    supplier.phone = args.phone;

    // make sure email is not too long
    require!(args.email.len() <= 40, SupplierError::SupplierEmailTooLong);
    supplier.email = args.email;

    // make sure routing number is 9 digits
    require!(args.routing_number.to_string().len() == 9, SupplierError::SupplierRoutingNumberLengthMismatch);
    supplier.routing_number = args.routing_number;

    Ok(())
}
