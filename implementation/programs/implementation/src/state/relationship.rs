use anchor_lang::prelude::*;

pub enum RelationshipType {
    Supplier,
    Contractor
}

pub enum RelationshipStatus {
    Active,
    Inactive
}

#[account]
pub struct Relationship {
    pub supplier: Pubkey,
    pub business_unit: Pubkey,
    pub relationship_type: RelationshipType,
    pub relationship_status: RelationshipStatus,
    pub relationship_start_date: String,
    pub relationship_end_date: Option<String>,
    pub initialization_date: i64,
    pub relationship_details: String,
    pub master_edition: Pubkey // master edition nft from which invoice transactions are minted
}

impl Relationship {
    pub const PREFIX: &'static [u8] = b"RELATIONSHIP";

    pub fn calc_space(args: &CreateRelationshipArgs) -> usize {
        32 + // supplier
        32 + // business_unit
        1 + // relationship_type
        1 + // relationship_status
        args.relationship_start_date.len() + // relationship_start_date
        args.relationship_end_date.len() + // relationship_end_date
        8 + // initialization_date
        args.relationship_details.len() + // relationship_details
        32 // master_edition
    }
}
