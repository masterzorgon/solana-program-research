use anchor_lang::prelude::*;

use crate::args::{ supplier_args::* };
use crate::state::{ supplier::* };
use crate::errors::{ supplier_errors::* };

#[derive(Accounts)]
pub struct UpdateSupplier<'info> {
    #[account(
        mut,
        seeds = [
            Supplier::PREFIX.as_ref(),
            b"_",
            &supplier.name.unwrap().as_bytes(),
            b"_",
            signer.key.as_ref()
        ],
        bump = supplier.bump,
        has_one = signer
    )]
    pub supplier: Account<'info, Supplier>,
    
    pub signer: Signer<'info>,
}

pub fn update_supplier(ctx: Context<UpdateSupplier>, args: UpdateSupplierArgs) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;

    if let Some(name) = args.name { 
        require!(name.len() <= 40, SupplierError::SupplierNameTooLong);
        supplier.name = name;
    }

    if let Some(address) = args.address { 
        require!(address.len() <= 40, SupplierError::SupplierAddressTooLong);
        supplier.address = address;
    }

    if let Some(phone) = args.phone { 
        require!(phone.len() <= 40 && phone.len() >= 10, SupplierError::SupplierPhoneLengthMismatch);
        supplier.phone = phone;
    }

    if let Some(email) = args.email { 
        require!(email.len() <= 40, SupplierError::SupplierEmailTooLong);
        supplier.email = email;
    }

    if let Some(routing_number) = args.routing_number { 
        require!(routing_number.len() == 12, SupplierError::SupplierRoutingNumberLengthMismatch);
        supplier.routing_number = routing_number;
    }

    Ok(())
}
