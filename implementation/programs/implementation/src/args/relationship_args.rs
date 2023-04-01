use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CreateRelationshipArgs {
    pub supplier_name: String,
    pub business_unit_name: String,

    pub relationship_type: RelationshipType,
    pub relationship_status: RelationshipStatus,
    pub relationship_start_date: String,
    pub relationship_end_date: Option<String>,
    pub relationship_details: Option<String>,
}
