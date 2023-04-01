use anchor_lang::prelude::*;

#[error_code]
pub enum RelationshipError {
    #[msg("The supplier relationships are too long")]
    SupplierRelationshipsTooLong,
    #[msg("You cannot create duplicate supplier relationships")]
    DuplicateSupplierRelationship,
    #[msg("You cannot create a relationship with a supplier that already exists")]
    ExistingSupplierRelationship,
    #[msg("The business unit name does not exist")]
    BusinessUnitNameNotValid,

    #[msg("The supplier name too long")]
    SupplierNameTooLong,
    #[msg("The business unit name is too long"))]
    BusinessUnitNameTooLong,
    #[msg("The relationship type is not valid")]
    RelationshipTypeNotValid,
    #[msg("The relationship status is not valid")]
    RelationshipStatusNotValid,
    #[msg("The relationship start date is not valid")]
    RelationshipStartDateLengthInvalid,
    #[msg("The relationship end date is not valid")]
    RelationshipEndDateLengthInvalid,
    #[msg("The relationship details are too long")]
    RelationshipDetailsTooLong
}
