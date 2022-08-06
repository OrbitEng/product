use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrbitProduct{
    pub price: f32,
    pub available: bool,
    pub seller: Pubkey,
    pub delivery_estimate: i8, // rough delivery ETA
}