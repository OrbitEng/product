use anchor_lang::prelude::*;
use market_accounts::{OrbitMarketAccount, AccountTransfer, program::OrbitMarketAccounts};
use crate::{ListingsStruct, ProductErrors, program::OrbitProduct};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// COMMISSIONS LISTINGS

#[derive(Accounts)]
pub struct CreateCommissionsListing<'info>{
    #[account(
        init,
        seeds = [
            b"vendor_listings".as_ref(),
            b"commission".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump,
        payer = wallet,
        space = 200
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        has_one = wallet
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,
    
    #[account(mut)]
    pub wallet: Signer<'info>,

    pub product_program: Program<'info, OrbitProduct>,

    pub accounts_program: Program<'info, OrbitMarketAccounts>,

    pub system_program: Program<'info, System>
}

// the idea here vendors can copy these old addresses into a new struct
pub fn init_commissions_listings_handler(ctx: Context<CreateCommissionsListing>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.vendor_account.key();

    // all available addresses (unused)
    ctx.accounts.vendor_listings.address_available = [u64::MAX,u64::MAX,u64::MAX,u64::MAX];

    // all available products (qty)
    ctx.accounts.vendor_listings.product_available = [0,0,0,0];

    market_accounts::cpi::add_vendor_commission_listings(
        CpiContext::new(
            ctx.accounts.accounts_program.to_account_info(),
            market_accounts::cpi::accounts::ModifyAccountLogs{
                market_account: ctx.accounts.vendor_account.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info()
            }
        )
    )
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// DIGITAL LISTINGS

#[derive(Accounts)]
pub struct CreateDigitalListing<'info>{
    #[account(
        init,
        seeds = [
            b"vendor_listings".as_ref(),
            b"digital".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump,
        payer = wallet,
        space = 200
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        has_one = wallet
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,
    
    #[account(mut)]
    pub wallet: Signer<'info>,

    pub product_program: Program<'info, OrbitProduct>,

    pub accounts_program: Program<'info, OrbitMarketAccounts>,

    pub system_program: Program<'info, System>
}

// the idea here vendors can copy these old addresses into a new struct
pub fn init_digital_listings_handler(ctx: Context<CreateDigitalListing>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.vendor_account.key();

    // all available addresses (unused)
    ctx.accounts.vendor_listings.address_available = [u64::MAX,u64::MAX,u64::MAX,u64::MAX];

    // all available products (qty)
    ctx.accounts.vendor_listings.product_available = [0,0,0,0];

    market_accounts::cpi::add_vendor_digital_listings(
        CpiContext::new(
            ctx.accounts.accounts_program.to_account_info(),
            market_accounts::cpi::accounts::ModifyAccountLogs{
                market_account: ctx.accounts.vendor_account.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info()
            }
        )
    )
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// PHYSICAL LISTINGS

#[derive(Accounts)]
pub struct CreatePhysicalListings<'info>{
    #[account(
        init,
        seeds = [
            b"vendor_listings".as_ref(),
            b"physical".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump,
        payer = wallet,
        space = 200
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        has_one = wallet
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,
    
    #[account(mut)]
    pub wallet: Signer<'info>,

    pub product_program: Program<'info, OrbitProduct>,

    pub accounts_program: Program<'info, OrbitMarketAccounts>,

    pub system_program: Program<'info, System>
}


// the idea here vendors can copy these old addresses into a new struct
pub fn init_physical_listings_handler(ctx: Context<CreatePhysicalListings>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.vendor_account.key();

    // all available addresses (unused)
    ctx.accounts.vendor_listings.address_available = [u64::MAX,u64::MAX,u64::MAX,u64::MAX];

    // all available products (qty)
    ctx.accounts.vendor_listings.product_available = [0,0,0,0];

    market_accounts::cpi::add_vendor_physical_listings(
        CpiContext::new(
            ctx.accounts.accounts_program.to_account_info(),
            market_accounts::cpi::accounts::ModifyAccountLogs{
                market_account: ctx.accounts.vendor_account.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info()
            }
        )
    )
}

////////////////////////////////////////////////////
/// PROD HELPERS

// I add checks on both accounts just cuz it makes me feel good inside
#[derive(Accounts)]
pub struct ModifyVendorListings<'info>{
    #[account(
        mut,
        constraint = vendor_listings.listings_owner == vendor_account.key()
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        has_one = wallet,
        address = vendor_listings.listings_owner
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,

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
    #[account(
        mut,
        constraint = vendor_listings.listings_owner == transfer_struct.source
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    pub transfer_struct: Account<'info, AccountTransfer>,
}

pub fn transfer_vendor_listings_ownership_handler(ctx: Context<TransferOwner>) -> Result<()>{
    ctx.accounts.vendor_listings.listings_owner = ctx.accounts.transfer_struct.destination;
    Ok(())
}

#[derive(Accounts)]
pub struct TransferAllOwner<'info>{
    #[account(
        mut,
        constraint = physical_vendor_listings.listings_owner == transfer_struct.source
    )]
    pub physical_vendor_listings: Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        constraint = digital_vendor_listings.listings_owner == transfer_struct.source
    )]
    pub digital_vendor_listings: Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        constraint = commission_vendor_listings.listings_owner == transfer_struct.source
    )]
    pub commission_vendor_listings: Box<Account<'info, ListingsStruct>>,

    pub transfer_struct: Account<'info, AccountTransfer>,
}

pub fn transfer_all_vendor_listings_ownership_handler(ctx: Context<TransferAllOwner>) -> Result<()>{
    ctx.accounts.physical_vendor_listings.listings_owner = ctx.accounts.transfer_struct.destination;
    ctx.accounts.digital_vendor_listings.listings_owner = ctx.accounts.transfer_struct.destination;
    ctx.accounts.commission_vendor_listings.listings_owner = ctx.accounts.transfer_struct.destination;
    Ok(())
}