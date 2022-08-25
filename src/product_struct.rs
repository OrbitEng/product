use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct OrbitProduct{
    pub currency: Pubkey, // 32
    pub price: u64, // 8
    pub available: bool, // 1
    pub seller: Pubkey, // 32
    pub delivery_estimate: u8, // rough delivery ETA // 1
}