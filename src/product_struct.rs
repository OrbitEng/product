use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OrbitProduct{
    pub name: String, // ideally max of 32 chars
    pub currency: Pubkey, // 32
    pub price: u64, // 8
    pub available: bool, // 1
    pub seller: Pubkey, // 32
    pub delivery_estimate: u8, // rough delivery ETA // 1
    pub media: String
}