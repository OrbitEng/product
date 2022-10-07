use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OrbitProduct{
    pub info: String, // 43
    pub owner_catalog: Pubkey, // 32
    pub index: u8,
    pub currency: Pubkey, // 32
    pub price: u64, // 8
    pub delivery_estimate: u8, // rough delivery ETA // 1
    pub media: String
}
// borsh
// 8 +  (1 + 43) | (32) | (8)  | (32) | (1) | 1 + 43

// disc b910f3a2 [185, 16, 243, 162, 235, 96, 85, 214]
#[account]
pub struct DigitalProduct{
    pub metadata: OrbitProduct,
    pub digital_file_type: DigitalFileTypes
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum DigitalFileTypes{
    Text,
    Video,
    Audio,
    Image,
    Folder
}