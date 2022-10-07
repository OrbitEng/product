use anchor_lang::prelude::*;
use market_accounts::{OrbitMarketAccount, AccountTransfer};
use crate::{ListingsStruct, ProductErrors};

#[derive(Accounts)]
#[instruction(market_type: String)]
pub struct CreateVendorListing<'info>{
    #[account(
        init,
        seeds = [
            b"listings_listings",
            market_type.as_bytes(),
            market_account.key().as_ref()
        ],
        bump,
        payer = wallet,
        space = 100
    )]
    pub vendor_listings:Box<Account<'info, ListingsStruct>>,

    #[account(
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump,
        seeds::program = market_accounts::ID
    )]
    pub market_account:Box<Account<'info, OrbitMarketAccount>>,
    
    #[account(
        mut,
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}

// the idea here vendors can copy these old addresses into a new struct
pub fn init_vendor_listings_handler(ctx: Context<CreateVendorListing>) -> Result<()>{
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
    pub vendor_listings:Box<Account<'info, ListingsStruct>>,

    #[account(
        address = vendor_listings.listings_owner
    )]
    pub wallet: Signer<'info>
}

pub fn list_product_handler(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    if ctx.accounts.vendor_listings.address_available[outer_ind as usize] & avail_ind == 0{
        return err!(ProductErrors::AddressUnavailable)
    }
    ctx.accounts.vendor_listings.address_available[outer_ind as usize] &= !(avail_ind as u64);
    ctx.accounts.vendor_listings.product_available[outer_ind as usize] |= avail_ind;
    Ok(())
}

pub fn unlist_product_handler(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    
    ctx.accounts.vendor_listings.address_available[outer_ind as usize] |= avail_ind;
    ctx.accounts.vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
    Ok(())
}

pub fn mark_prod_available_handler(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    ctx.accounts.vendor_listings.product_available[outer_ind as usize] |= avail_ind as u64;
    Ok(())
}

pub fn mark_prod_unavailable_handler(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    
    ctx.accounts.vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
    Ok(())
}


///////////////////////////////////////////////////
/// ACCOUNT TRANSFER UTILS

#[derive(Accounts)]
pub struct TransferOwner<'info>{
    #[account(
        mut,
        constraint = vendor_listings.listings_owner == source_account.wallet
    )]
    pub vendor_listings:Box<Account<'info, ListingsStruct>>,

    #[account(
        address = transfer_struct.source
    )]
    pub source_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = transfer_struct.destination
    )]
    pub destination_account:Box<Account<'info, OrbitMarketAccount>>,
    
    pub transfer_struct: Account<'info, AccountTransfer>,
}

pub fn transfer_vendor_listings_ownership_handler(ctx: Context<TransferOwner>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.destination_account.wallet;
    Ok(())
}

#[derive(Accounts)]
pub struct TransferAllOwner<'info>{
    #[account(
        mut,
        seeds = [
            b"listings_listings",
            b"physical",
            source_account.key().as_ref()
        ],
        bump,
        constraint = physical_vendor_listings.listings_owner == source_account.wallet
    )]
    pub physical_vendor_listings:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        seeds = [
            b"listings_listings",
            b"digital",
            source_account.key().as_ref()
        ],
        bump,
        constraint = digital_vendor_listings.listings_owner == source_account.wallet
    )]
    pub digital_vendor_listings:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        seeds = [
            b"listings_listings",
            b"commission",
            source_account.key().as_ref()
        ],
        bump,
        constraint = commission_vendor_listings.listings_owner == source_account.wallet
    )]
    pub commission_vendor_listings:Box<Account<'info, ListingsStruct>>,

    #[account(
        address = transfer_struct.source
    )]
    pub source_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = transfer_struct.destination
    )]
    pub destination_account:Box<Account<'info, OrbitMarketAccount>>,
    
    pub transfer_struct: Account<'info, AccountTransfer>,
}

pub fn transfer_all_vendor_listings_ownership_handler(ctx: Context<TransferAllOwner>) -> Result<()>{
    ctx.accounts.physical_vendor_listings.listings_owner = ctx.accounts.destination_account.wallet;
    ctx.accounts.digital_vendor_listings.listings_owner = ctx.accounts.destination_account.wallet;
    ctx.accounts.commission_vendor_listings.listings_owner = ctx.accounts.destination_account.wallet;
    Ok(())
}

#[derive(Accounts)]
pub struct VendorForceChangeOwner<'info>{
    #[account(
        mut,
        constraint = vendor_listings.listings_owner == wallet.key()
    )]
    pub vendor_listings:Box<Account<'info, ListingsStruct>>,

    pub new_wallet: SystemAccount<'info>,
    
    pub wallet: Signer<'info>,
}

pub fn vendor_force_change_ownership_handler(ctx: Context<VendorForceChangeOwner>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.new_wallet.key();
    Ok(())
}

