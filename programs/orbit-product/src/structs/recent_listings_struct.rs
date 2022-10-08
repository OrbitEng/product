use anchor_lang::prelude::*;

#[account]
pub struct RecentMarketListings{
    pub market_type: String,
    pub pubkeys: [Pubkey; 25],
    pub index: u8
}