use anchor_lang::{
    prelude::*
};

use orbit_addresses::{
    PHYSICAL_ADDRESS,
    DIGITAL_ADDRESS,
    COMMISSION_ADDRESS,
    SEARCH_ADDRESS
};

use crate::{
    ListingsStruct,
    mark_prod_available_handler,
    mark_prod_unavailable_handler, ProductErrors
};

////////////////////////////////////////////
/// GENERAL

#[derive(Accounts)]
pub struct UpdateProductField<'info>{
    #[account(
        mut,
        owner = crate::ID,
        constraint = product.data_len() == 250
    )]
    /// CHECK: we check the program owns it
    pub product: AccountInfo<'info>,

    #[account(
        mut,
        constraint = product.try_borrow_data()?[55..87] == vendor_listings.key().to_bytes()
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub wallet: Signer<'info>,
}

pub fn update_info_handler(ctx: Context<UpdateProductField>, info: String) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    if info.len() != 43{
        return err!(ProductErrors::InvalidParameter);
    }
    let inf_ser = info.try_to_vec()?;
    mut_data[8..inf_ser.len()+8].copy_from_slice(&inf_ser);
    ctx.accounts.product.exit(&crate::ID)
}

pub fn update_price_handler(ctx: Context<UpdateProductField>, price: u64) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    mut_data[88..96].copy_from_slice(&price.try_to_vec()?);
    ctx.accounts.product.exit(&crate::ID)
}

pub fn update_delivery_estimate_handler(ctx: Context<UpdateProductField>, delivery_estimate: u8) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    mut_data[96] = delivery_estimate;
    ctx.accounts.product.exit(&crate::ID)
}

pub fn update_media_handler(ctx: Context<UpdateProductField>, link: String) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    if link.len() != 43{
        return err!(ProductErrors::InvalidParameter);
    }
    let link_ser = link.try_to_vec()?;
    mut_data[97..link_ser.len()+97].copy_from_slice(&link_ser);
    ctx.accounts.product.exit(&crate::ID)
}

pub fn mark_available_handler(ctx: Context<UpdateProductField>) -> Result<()>{
    mark_prod_available_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.try_borrow_data()?[87])
}

pub fn mark_unavailable_handler(ctx: Context<UpdateProductField>) -> Result<()>{
    mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.try_borrow_data()?[87])
}

pub fn mark_searchable_handler(ctx: Context<UpdateProductField>, ref_bump: u8) -> Result<()>{
    let mut mut_data = ctx.accounts.product.try_borrow_mut_data()?;
    mut_data[144] = ref_bump;
    ctx.accounts.product.exit(&crate::ID)
}

#[derive(Accounts)]
pub struct UpdateProductFieldInternal<'info>{
    #[account(
        mut,
        owner = crate::ID,
        constraint = product.data_len() == 250
    )]
    /// CHECK: we check the program owns it
    pub product: AccountInfo<'info>,

    #[account(
        mut,
        constraint = product.try_borrow_data()?[55..87] == vendor_listings.key().to_bytes()
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

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
            (caller.key().to_bytes() == PHYSICAL_ADDRESS) ||
            (caller.key().to_bytes() == DIGITAL_ADDRESS) ||
            (caller.key().to_bytes() == COMMISSION_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}


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