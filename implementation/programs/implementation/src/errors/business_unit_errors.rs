use anchor_lang::prelude::*;

#[error_code]
pub struct BusinessUnitError {
    #[msg("Company name size invalid")]
    CompanyNameSizeInvalid,

    #[msg("Address size invalid")]
    AddressSizeInvalid,
    
    #[msg("Business unit name size invalid")]
    BusinessUnitNameSizeInvalid,
    
    #[msg("Supervisor name size invalid")]
    SupervisorNameSizeInvalid,
    
    #[msg("Email size invalid")]
    EmailSizeInvalid,
    
    #[msg("Phone size invalid")]
    PhoneSizeInvalid,
    
    #[msg("Routing number size invalid")]
    RoutingNumberSizeInvalid
}
