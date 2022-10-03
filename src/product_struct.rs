use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OrbitProduct{
    pub info: String, // 43
    pub owner_catalog: Pubkey, // 32

    pub currency: Pubkey, // 32
    pub price: u64, // 8
    pub delivery_estimate: u8, // rough delivery ETA // 1
    pub media: String
}

// borsh
// 8 +  (1 + 43) | (32) | (8)  | (32) | (1) | 1 + 43