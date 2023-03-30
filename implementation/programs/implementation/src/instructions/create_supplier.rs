use anchor_lang::prelude::*;

use crate::state::{
    supplier::Supplier,
};

use crate::errors::{
    supplier_errors::SupplierError,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SupplierArgs {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: u32,
}

#[derive(Accounts)]
#[instruction(args: SupplierArgs)]
pub struct CreateSupplier<'info> {
    #[account(
        init_if_needed, // initializes the account if it does not exist
        seeds = [
            Supplier::PREFIX.as_ref(),
            b"_",
            &args.name.as_bytes(),
            b"_",
            signer.key.as_ref()
        ],
        bump,
        payer = signer,
        space = Supplier::calc_space(&args)
    )]
    pub supplier: Account<'info, Supplier>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_supplier(ctx: Context<CreateSupplier>, args: SupplierArgs) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;

    let (addr, bump) = Pubkey::find_program_address(
        &[
            Supplier::PREFIX.as_ref(),
            b"_",
            args.name.as_bytes().as_ref(),
            b"_",
            ctx.accounts.signer.key.as_ref()
        ],
        ctx.program_id
    );
    
    supplier.bump = bump;
    supplier.identifier = addr;

    supplier.invoices = vec![];
    supplier.total_transactions = 0;

    // make sure name is not too long
    require!(
        args.name.len() <= 40, 
        SupplierError::SupplierNameTooLong
    );
    supplier.name = args.name;

    // make sure address is not too long
    require!(
        args.address.len() <= 40, 
        SupplierError::SupplierAddressTooLong
    );
    supplier.address = args.address;

    // make sure phone is not too long
    require!(
        args.phone.len() <= 40 && args.phone.len() >= 10, 
        SupplierError::SupplierPhoneTooLong
    );
    supplier.phone = args.phone;

    // make sure email is not too long
    require!(
        args.email.len() <= 40, 
        SupplierError::SupplierEmailTooLong
    );
    supplier.email = args.email;

    // make sure routing number is 9 digits
    require!(
        args.routing_number.to_string().len() == 9, 
        SupplierError::SupplierRoutingNumberLengthMismatch
    );
    supplier.routing_number = args.routing_number;

    msg!("Supplier created successfully!: {}", supplier.name);

    Ok(())
}
