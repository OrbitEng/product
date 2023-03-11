use anchor_lang::{
    prelude::*
};

use orbit_addresses::SEARCH_ADDRESS;

use crate::ListingsStruct;

/////////////////////////////////
/// SEARCH CPI

#[derive(Accounts)]
pub struct SearchSetting<'info>{
    #[account(
        mut,
        owner = crate::ID,
        constraint = product.data_len() == 250
    )]
    /// CHECK: we check the program owns it
    pub product: AccountInfo<'info>,

    #[account(
        seeds = [
            b"market_authority"
        ],
        bump,
        seeds::program = caller.key()
    )]
    pub caller_auth: Signer<'info>,

    #[account(
        executable,
        constraint = 
            (caller.key().to_bytes() == SEARCH_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}

pub fn set_searchable_handler(ctx: Context<SearchSetting>, bucket_number: u8) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    mut_data[144] = bucket_number;
    ctx.accounts.product.exit(&crate::id())
}

pub fn set_unsearchable_handler(ctx: Context<SearchSetting>, bucket_number: u8) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    mut_data[144] = bucket_number;
    ctx.accounts.product.exit(&crate::id())
}


///////////////////////////////////////
/// EMERGENCY FUCKUP USE ONLY

#[derive(Accounts)]
pub struct FlushListings<'info>{
    #[account(
        mut
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub wallet: Signer<'info>
}

pub fn flush_listings_handler(ctx: Context<FlushListings>) -> Result<()>{
    ctx.accounts.vendor_listings.address_available = [u64::MAX,u64::MAX,u64::MAX,u64::MAX];
    Ok(())
}