use anchor_lang::prelude::*;

#[account]
pub struct RecentMarketListings{
    pub market_type: [u8; 8],
    pub pubkeys: [Pubkey; 25],
    pub index: u8
}