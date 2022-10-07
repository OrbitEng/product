use anchor_lang::prelude::*;

#[account]
pub struct ListingsStruct{
    pub listings_owner: Pubkey,
    pub address_available: [u64; 4], // 32
    pub product_available: [u64; 4], // 32
}