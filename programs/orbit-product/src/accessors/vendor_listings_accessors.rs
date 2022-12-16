use anchor_lang::prelude::*;
use market_accounts::OrbitMarketAccount;
use crate::{ListingsStruct, ProductErrors};

#[derive(Accounts)]
#[instruction(market_type: String)]
pub struct CreateVendorListing<'info>{
    #[account(
        init,
        seeds = [
            b"vendor_listings",
            market_type.as_bytes(),
            vendor_account.voter_id.to_bytes()
        ],
        bump,
        payer = wallet,
        space = 200
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    pub vendor_account: Account<'info, OrbitMarketAccount>,
    
    #[account(mut)]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}

// the idea here vendors can copy these old addresses into a new struct
pub fn init_vendor_listings_handler(ctx: Context<CreateVendorListing>, _market_type: String) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.wallet.key();
    
    // all available addresses (unused)
    ctx.accounts.vendor_listings.address_available = [u64::MAX,u64::MAX,u64::MAX,u64::MAX];

    // all available products (qty)
    ctx.accounts.vendor_listings.product_available = [0,0,0,0];
    Ok(())
}

////////////////////////////////////////////////////
/// PROD HELPERS

#[derive(Accounts)]
pub struct ModifyVendorListings<'info>{
    #[account(mut)]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        address = vendor_listings.listings_owner
    )]
    pub wallet: Signer<'info>
}

pub fn list_product_handler(vendor_listings: &mut Account<ListingsStruct>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    if vendor_listings.address_available[outer_ind as usize] & avail_ind == 0{
        return err!(ProductErrors::AddressUnavailable)
    }
    vendor_listings.address_available[outer_ind as usize] &= !(avail_ind as u64);
    vendor_listings.product_available[outer_ind as usize] |= avail_ind;
    Ok(())
}

pub fn mark_prod_available_handler(vendor_listings: &mut Account<ListingsStruct>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    vendor_listings.product_available[outer_ind as usize] |= avail_ind as u64;
    Ok(())
}

pub fn mark_prod_unavailable_handler(vendor_listings: &mut Account<ListingsStruct>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    
    vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
    Ok(())
}


///////////////////////////////////////////////////
/// ACCOUNT TRANSFER UTILS

#[derive(Accounts)]
pub struct TransferOwner<'info>{
    #[account(mut)]
    pub vendor_listings: Account<'info, ListingsStruct>,

    pub destination_wallet: SystemAccount<'info>,

    #[account(
        address = vendor_listings.listings_owner
    )]
    pub wallet: Signer<'info>
}

pub fn transfer_vendor_listings_ownership_handler(ctx: Context<TransferOwner>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.destination_wallet.key();
    Ok(())
}

#[derive(Accounts)]
pub struct TransferAllOwner<'info>{
    #[account(
        mut,
        has_one = listings_owner
    )]
    pub physical_vendor_listings: Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        has_one = listings_owner
    )]
    pub digital_vendor_listings: Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        has_one = listings_owner
    )]
    pub commission_vendor_listings: Box<Account<'info, ListingsStruct>>,

    pub destination_wallet: SystemAccount<'info>,

    pub listings_owner: Signer<'info>
}

pub fn transfer_all_vendor_listings_ownership_handler(ctx: Context<TransferAllOwner>) -> Result<()>{
    ctx.accounts.physical_vendor_listings.listings_owner = ctx.accounts.destination_wallet.key();
    ctx.accounts.digital_vendor_listings.listings_owner = ctx.accounts.destination_wallet.key();
    ctx.accounts.commission_vendor_listings.listings_owner = ctx.accounts.destination_wallet.key();
    Ok(())
}