use anchor_lang::prelude::*;

// borsh:    8 +  (1 + 43) | (32) | (8)  | (32) | (1) | 1 + 43
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OrbitProduct{ // size 170
    pub info: String, // 43 :0-52
    pub owner_catalog: Pubkey, // 32 : 52-84
    pub index: u8, // :84 - 85
    pub currency: Pubkey, // 32 :85 - 117
    pub price: u64, // 8 : 117 - 125
    pub delivery_estimate: u8, // rough delivery ETA // 1 // 125 - 126
    pub media: String // :126-169
}

/// DIGITAL
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

/// PHYSICAL
// disc a81c2d2c [168, 28, 45, 44, 211, 241, 238, 140]
#[account]
pub struct PhysicalProduct{
    pub metadata: OrbitProduct, // 38 as of beta
    pub quantity: u32, // quantity per purchase // 4
}

/// COMMISSION
// disc ee26edf0 [238, 38, 237, 240, 132, 29, 235, 255]
#[account]
pub struct CommissionProduct{
    pub metadata: OrbitProduct
}