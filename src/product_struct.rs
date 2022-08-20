use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct OrbitProduct{
    pub currency: Pubkey,
    pub price: u64, // 4
    pub available: bool, // 1
    pub seller: Pubkey, // 32
    pub delivery_estimate: u8, // rough delivery ETA // 1
}