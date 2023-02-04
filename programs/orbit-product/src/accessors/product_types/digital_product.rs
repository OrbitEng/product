use anchor_lang::prelude::*;
use market_accounts::OrbitMarketAccount;
use orbit_addresses::DIGITAL_ADDRESS;

use crate::{
    structs::DigitalProduct,
    structs::ListingsStruct,
    structs::OrbitProductStruct,
    structs::RecentMarketListings,
    structs::DigitalFileTypes,

    errors::ProductErrors,

    accessors::vendor_listings_accessors::list_product_handler,
    accessors::vendor_listings_accessors::mark_prod_available_handler,
    accessors::vendor_listings_accessors::mark_prod_unavailable_handler,
    accessors::recent_listings_accessors::edit_recent_listings_handler,
};

/////////////////////////////////////////////////
/// DIGITAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProductStruct)]
pub struct ListDigitalProduct<'info>{
    
    #[account(
        init,
        space = 250,
        payer = wallet,
        seeds = [
            b"digital_product",
            vendor_listings.key().as_ref(),
            &[prod_in.index]
        ],
        bump,
        constraint = prod_in.owner_catalog == vendor_account.voter_id
    )]
    pub prod: Account<'info, DigitalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"digital".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        has_one = wallet
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut
    )]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UnlistDigitalProduct<'info>{
    #[account(
        mut,
        constraint = prod.metadata.owner_catalog == vendor_account.voter_id,
        close = wallet
    )]
    /// CHECK: we do owner check
    pub prod: Account<'info, DigitalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"digital".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        has_one = wallet
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut
    )]
    pub wallet: Signer<'info>
}

pub fn list_digital_handler(ctx: Context<ListDigitalProduct>, prod: OrbitProductStruct, file_type: DigitalFileTypes)-> Result<()> {
    list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;

    ctx.accounts.prod.metadata = prod;
    ctx.accounts.prod.digital_file_type = file_type;

    if ctx.remaining_accounts.len() == 1{
        let addr = Pubkey::find_program_address(&[
            b"recent_listings".as_ref(),
            b"digital".as_ref()
        ], 
        &crate::ID);

        if ctx.remaining_accounts[0].key() != addr.0{
            return err!(ProductErrors::InvalidCatalogType)
        };

        let recent_catalog = &mut Account::<RecentMarketListings>::try_from(&ctx.remaining_accounts[0])?;
        edit_recent_listings_handler(recent_catalog, ctx.accounts.prod.to_account_info())?;
        recent_catalog.exit(&crate::ID)?;
    }
    Ok(())
}

pub fn unlist_digital_handler(ctx: Context<UnlistDigitalProduct>) -> Result<()>{
    //https://github.com/coral-xyz/anchor/blob/master/lang/src/common.rs

    let listings_index = ctx.accounts.prod.metadata.index;

    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    
    ctx.accounts.vendor_listings.address_available[outer_ind as usize] |= avail_ind;
    ctx.accounts.vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
    
    Ok(())
}

/////////////////////
/// DIGITAL ONLY

#[derive(Accounts)]
pub struct SetFileType<'info>{
    #[account(
        mut,
        seeds = [
            b"digital_product",
            vendor_listings.key().as_ref(),
            &[product.metadata.index]
        ],
        bump,
    )]
    pub product: Account<'info, DigitalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"digital".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = vendor_account.wallet
    )]
    pub wallet: Signer<'info>,
}

pub fn set_file_type_handler(ctx: Context<SetFileType>, file_type: DigitalFileTypes) -> Result<()>{    
    ctx.accounts.product.digital_file_type = file_type;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateDigitalQuantityInternal<'info>{
    #[account(
        mut
    )]
    pub product: Account<'info, DigitalProduct>,

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
        address = Pubkey::new(DIGITAL_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}

pub fn digital_increment_times_sold_handler(ctx: Context<UpdateDigitalQuantityInternal>) -> Result<()>{
    ctx.accounts.product.metadata.times_sold += 1;
    Ok(())
}

////////////////////////////////////////////
/// GENERAL

#[derive(Accounts)]
pub struct UpdateDigitalProductField<'info>{
    #[account(
        mut,
        constraint = product.metadata.owner_catalog == vendor_account.voter_id
    )]
    pub product: Account<'info, DigitalProduct>,

    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = vendor_account.wallet
    )]
    pub wallet: Signer<'info>,
}

pub fn digital_update_info_handler(ctx: Context<UpdateDigitalProductField>, info: String) -> Result<()>{
    ctx.accounts.product.metadata.info = info;
    Ok(())
}

pub fn digital_update_price_handler(ctx: Context<UpdateDigitalProductField>, price: u64) -> Result<()>{
    ctx.accounts.product.metadata.price = price;
    Ok(())
}

pub fn digital_update_delivery_estimate_handler(ctx: Context<UpdateDigitalProductField>, delivery_estimate: u8) -> Result<()>{
    ctx.accounts.product.metadata.delivery_estimate = delivery_estimate;
    Ok(())
}

pub fn digital_update_media_handler(ctx: Context<UpdateDigitalProductField>, link: String) -> Result<()>{
    ctx.accounts.product.metadata.media = link;
    Ok(())
}

pub fn mark_digital_searchable_handler(ctx:Context<UpdateDigitalProductField>) -> Result<()>{
    ctx.accounts.product.metadata.search_indexed = true;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateDigitalProductListingField<'info>{
    #[account(
        mut,
        seeds = [
            b"digital_product",
            vendor_listings.key().as_ref(),
            &[product.metadata.index]
        ],
        bump,
    )]
    pub product: Account<'info, DigitalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"digital".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = vendor_account.wallet
    )]
    pub wallet: Signer<'info>,
}

pub fn digital_mark_available_handler(ctx: Context<UpdateDigitalProductListingField>) -> Result<()>{
    mark_prod_available_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)
}

pub fn digital_mark_unavailable_handler(ctx: Context<UpdateDigitalProductListingField>) -> Result<()>{
    mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)
}
