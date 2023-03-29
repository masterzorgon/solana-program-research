use anchor_lang::prelude::*;

use crate::state::{
    supplier::Supplier,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateSupplierArgs {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub routing_number: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateSupplierArgs)]
pub struct CreateSupplier<'info> {
    #[account(
        init,
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

pub fn create_supplier(ctx: Context<CreateSupplier>, args: CreateSupplierArgs) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;

    let (addr, bump) = Pubkey::find_program_address(
        &[
            Supplier::PREFIX.as_ref(),
            args.name.as_bytes().as_ref(),
            ctx.accounts.signer.key.as_ref()
        ],
        ctx.program_id
    );

    supplier.bump = bump;
    supplier.identifier = addr;
    supplier.name = args.name;
    supplier.address = args.address;
    supplier.phone = args.phone;
    supplier.email = args.email;
    supplier.routing_number = args.routing_number;
    supplier.invoices = vec![];
    supplier.total_transactions = 0;

    msg!("Supplier created successfully!: {}", supplier.name);

    Ok(())
}