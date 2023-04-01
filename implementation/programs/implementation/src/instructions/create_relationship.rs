use anchor_lang::prelude::*;

use crate::args::{ relationship_args::* };
use crate::errors::{ relationship_errors::* };
use crate::state::{ relationship::* };

#[derive(Accounts)]
#[instruction(args: CreateRelationshipArgs)]
pub struct CreateRelationship<'info> {
    #[account(
        init,
        seeds = [
            Relationship::PREFIX.as_ref(),
            b"_",
            &args.supplier_name.as_bytes(),
            b"_",
            &args.business_unit_name.as_bytes(),
            b"_",
            signer.key.as_ref()
        ],
        bump,
        payer = signer,
        space = Relationship::calc_space(&args)
    )]
    pub relationship: Account<'info, Supplier>,

    pub signer: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

// create relationship account
pub fn create_relationship(ctx: Context<CreateRelationship>, args: CreateRelationshipArgs) -> Result<()> {
    let relationship = &mut ctx.accounts.relationship;

    // find supplier PDA using `supplier_name`
    // find business unit PDA using `business_unit_name`

    // assign PDAs to relationship account

    // create a Master Edition NFT for the relationship

    require!(
        args.relationship_type == RelationshipType::Supplier ||
        args.relationship_type == RelationshipType::Contractor, 
        RelationshipError::RelationshipTypeNotValid
    );
    relationship.relationship_type = args.relationship_type;

    require!(
        args.relationship_status == RelationshipStatus::Active ||
        args.relationship_status == RelationshipStatus::Inactive, 
        RelationshipError::RelationshipStatusInvalid
    );
    relationship.relationship_status = args.relationship_status;

    require!(
        args.relationship_start_date.len() == 10, // YYYY-MM-DD
        RelationshipError::RelationshipStartDateLengthInvalid
    );
    relationship.relationship_start_date = args.relationship_start_date;

    if let Some(relationship_end_date) = args.relationship_end_date {
        require!(
            relationship_end_date.len() == 10, // YYYY-MM-DD
            RelationshipError::RelationshipEndDateLengthInvalid
        );
        relationship.relationship_end_date = Some(relationship_end_date);
    }

    if let Some(relationship_details) = args.relationship_details {
        require!(
            relationship_details.len() <= 250,
            RelationshipError::RelationshipDetailsTooLong
        );
        relationship.relationship_details = Some(relationship_details);
    }

    Ok(())
}

pub fn assign_relationships(ctx: Context<CreateRelationship>, args: Vec<CreateRelationshipArgs>) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;

    // check that args is not empty
    require!(args.len() > 0, RelationshipError::SupplierRelationshipsTooLong);

    // make sure that each relationship is unique
    let mut unique = true;
    for i in 0..args.len() {
        for j in 0..args.len() {
            if i != j {
                require!(
                    args[i].supplier_name != args[j].supplier_name || 
                    args[i].business_unit_name != args[j].business_unit_name || 
                    RelationshipError::DuplicateSupplierRelationship
                );
            }
        }
    }

    // make sure that the relationship does not already exist with the supplier
    for relationship in args.iter() {
        for existing_relationship in supplier.relationships.iter() {
            require!(
                relationship.business_unit_name != existing_relationship.business_unit_name,
                RelationshipError::ExistingSupplierRelationship
            );
        }
    }

    // make sure that data values for each relationship are appropriate
    for relationship in args.iter() {
        require!(relationship.supplier_name.len() <= 40, RelationshipError::SupplierNameTooLong);
        require!(relationship.business_unit_name.len() <= 40, RelationshipError::BusinessUnitNameTooLong);

        // make sure that each business unit name matches the acceptable list of business units
        check_business_units(&args);
    }

    // for each relationship, create a master edition nft

    // add the relationships to the supplier

    Ok(())
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
            RelationshipError::BusinessUnitNameNotValid
        );
    }

    Ok(())
}
