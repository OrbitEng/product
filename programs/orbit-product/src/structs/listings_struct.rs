use anchor_lang::prelude::*;

// must be pubkey or else we cant find source acc
#[account]
pub struct ListingsStruct{
    pub listings_owner: Pubkey,
    pub address_available: [u64; 4], // 32
    pub product_available: [u64; 4], // 32
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq)]
pub enum ListingsType {
    Commissions,
    Digital,
    Physical
}