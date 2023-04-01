use anchor_lang::prelude::*;

use crate::args::{ supplier_args::* };
use crate::errors::{ supplier_errors::* };
use crate::state::{ supplier::* };

#[derive(Accounts)]
pub struct CreateRelationship<'info> {

}

pub fn create_relationship(ctx: Context<CreateRelationship>, args: Vec<RelationshipArgs>) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;

    // check that args is not empty
    require(args.len() > 0, SupplierError::SupplierRelationshipsTooLong);

    // make sure that each relationship is unique
    let mut unique = true;
    for i in 0..args.len() {
        for j in 0..args.len() {
            if i != j {
                require!(
                    args[i].supplier_name != args[j].supplier_name || 
                    args[i].business_unit_name != args[j].business_unit_name || 
                    SupplierError::DuplicateSupplierRelationship
                );
            }
        }
    }

    // make sure that the relationship does not already exist with the supplier
    for relationship in args.iter() {
        for existing_relationship in supplier.relationships.iter() {
            require!(
                relationship.business_unit_name != existing_relationship.business_unit_name,
                SupplierError::ExistingSupplierRelationship
            );
        }
    }

    // make sure that data values for each relationship are appropriate
    for relationship in args.iter() {
        require!(relationship.supplier_name.len() <= 40, SupplierError::SupplierNameTooLong);
        require!(relationship.business_unit_name.len() <= 40, SupplierError::BusinessUnitNameTooLong);

        // make sure that each business unit name matches the acceptable list of business units
        check_business_units(&args);
    }

    // add the relationships to the supplier
    for relationship in args {
        supplier.relationships.push(relationship);
    }
}

fn check_business_units(list: &Vec<RelationshipArgs>) {
    let valid_business_units = vec![
        "Legacy Tyson",
        "Tyson Fresh Meats",
        "Advance Pierre",
        "Hillshire",
        "Keystone",
        "Original Philly"
    ];

    for relationship in list.iter() {
        require!(
            valid_business_units.contains(relationship.business_unit_name),
            SupplierError::BusinessUnitNameNotValid
        );
    }
}