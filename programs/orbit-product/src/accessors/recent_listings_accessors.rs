use anchor_lang::prelude::*;

use crate::{
    RecentMarketListings
};

#[derive(Accounts)]
pub struct InitMarketRecentListings<'info>{
    #[account(
        init,
        space = 1000,
        seeds = [
            b"recent_listings".as_ref(),
            b"physical".as_ref()
        ],
        bump,
        payer = payer
    )]
    pub physical_recent_listings: Box<Account<'info, RecentMarketListings>>,

    #[account(
        init,
        space = 1000,
        seeds = [
            b"recent_listings".as_ref(),
            b"digital".as_ref()
        ],
        bump,
        payer = payer
    )]
    pub digital_recent_listings: Box<Account<'info, RecentMarketListings>>,

    #[account(
        init,
        space = 1000,
        seeds = [
            b"recent_listings".as_ref(),
            b"commission".as_ref()
        ],
        bump,
        payer = payer
    )]
    pub commission_recent_listings: Box<Account<'info, RecentMarketListings>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn init_recent_listings_handler(_ctx: Context<InitMarketRecentListings>) -> Result<()>{
    Ok(())
}


pub fn edit_recent_listings_handler(recent_catalog: &mut Account<RecentMarketListings>, new_prod: AccountInfo) -> Result<()>{
    let index = recent_catalog.index;
    recent_catalog.pubkeys[index as usize] = new_prod.key();
    recent_catalog.index = (index+1) % 25;
    Ok(())
}