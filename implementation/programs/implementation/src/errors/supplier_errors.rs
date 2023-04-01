use anchor_lang::prelude::*;

#[error_code]
pub enum SupplierError {
    #[msg("The supplier already exists")]
    SupplierAlreadyExists,
    #[msg("The supplier name is too long")]
    SupplierNameTooLong,
    #[msg("The supplier address is too long")]
    SupplierAddressTooLong,
    #[msg("The supplier phone is too long")]
    SupplierPhoneLengthMismatch,
    #[msg("The supplier email is too long")]
    SupplierEmailTooLong,
    #[msg("The supplier routing number does not meet the length requirements")]
    SupplierRoutingNumberLengthMismatch,
    #[msg("The supplier relationships are too long")]
    SupplierRelationshipsTooLong,
    #[msg("You cannot create duplicate supplier relationships")]
    DuplicateSupplierRelationship,
    #[msg("You cannot create a relationship with a supplier that already exists")]
    ExistingSupplierRelationship,
    #[msg("The business unit name does not exist")]
    BusinessUnitNameNotValid
}
