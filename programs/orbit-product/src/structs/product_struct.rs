use anchor_lang::prelude::*;

// borsh:    8 +  (4 + 43) | (32) | (8)  | (32) | (1) | 4 + 43
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OrbitProductStruct{ // size 170 :0-8
    pub info: String, // 43 :8-12-55
    pub owner_catalog: Pubkey, // 32 : 55-87
    pub index: u8, // :87 - 88
    pub price: u64, // 8 : 88 - 96
    pub delivery_estimate: u8, // rough delivery ETA :96-97
    pub media: String, // :97-101-144
    pub search_bucket: u8,
}

/// DIGITAL
// disc b910f3a2 [185, 16, 243, 162, 235, 96, 85, 214]
#[account]
pub struct DigitalProduct{
    pub metadata: OrbitProductStruct,
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
    pub metadata: OrbitProductStruct, // 38 as of beta
    pub quantity: u32, // quantity per purchase // 4
}

/// COMMISSION
// disc ee26edf0 [238, 38, 237, 240, 132, 29, 235, 255]
#[account]
pub struct CommissionProduct{
    pub metadata: OrbitProductStruct
}