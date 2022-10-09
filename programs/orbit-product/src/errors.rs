use anchor_lang::prelude::*;

#[error_code]
pub enum ProductErrors{
    #[msg("requested PDA is unavailable")]
    AddressUnavailable,
    #[msg("given address is not a recent catalog")]
    InvalidCatalogType
}