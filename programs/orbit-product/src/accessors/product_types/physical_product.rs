use anchor_lang::{
    prelude::*
};
use market_accounts::OrbitMarketAccount;
use orbit_addresses::PHYSICAL_ADDRESS;

use crate::{
    structs::PhysicalProduct,
    structs::ListingsStruct,
    structs::OrbitProductStruct,
    structs::RecentMarketListings,

    errors::ProductErrors,

    accessors::vendor_listings_accessors::list_product_handler,
    accessors::vendor_listings_accessors::mark_prod_available_handler,
    accessors::vendor_listings_accessors::mark_prod_unavailable_handler,
    accessors::recent_listings_accessors::edit_recent_listings_handler,
};
////////////////////////////////////////////////
/// PHYSICAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProductStruct)]
pub struct ListPhysicalProduct<'info>{

    #[account(
        init,
        space = 250, // 106 + 8. leave room for adjustment during launch
        payer = wallet,

        seeds = [
            b"physical_product".as_ref(),
            vendor_listings.key().as_ref(),
            &[prod_in.index]
        ],
        bump,
        constraint = prod_in.owner_catalog == vendor_account.voter_id
    )]
    pub prod: Account<'info, PhysicalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"physical".as_ref(),
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
pub struct UnlistPhysicalProduct<'info>{
    #[account(
        mut,
        constraint = prod.metadata.owner_catalog == vendor_account.voter_id,
        close = wallet
    )]
    /// CHECK: we do owner check
    pub prod: Account<'info, PhysicalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"physical".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        has_one = wallet
    )]
    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(mut)]
    pub wallet: Signer<'info>
}

pub fn list_physical_handler(ctx: Context<ListPhysicalProduct>, prod: OrbitProductStruct, quantity: u32) -> Result<()>{
    list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;

    ctx.accounts.prod.metadata = prod;
    ctx.accounts.prod.quantity = quantity;

    if ctx.remaining_accounts.len() == 1{
        let addr = Pubkey::find_program_address(&[
            b"recent_listings".as_ref(),
            b"physical".as_ref()
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


pub fn unlist_physical_handler(ctx: Context<UnlistPhysicalProduct>) -> Result<()>{
    //https://github.com/coral-xyz/anchor/blob/master/lang/src/common.rs

    let listings_index = ctx.accounts.prod.metadata.index;

    let avail_ind = 1<<(listings_index%64);
    let outer_ind = listings_index/64;
    
    ctx.accounts.vendor_listings.address_available[outer_ind as usize] |= avail_ind;
    ctx.accounts.vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
    
    Ok(())
}



/////////////////////////
/// PHYS ONLY
#[derive(Accounts)]
pub struct UpdatePhysicalQuantity<'info>{
    #[account(
        mut,
        seeds = [
            b"physical_product",
            vendor_listings.key().as_ref(),
            &[product.metadata.index]
        ],
        bump,
    )]
    pub product: Account<'info, PhysicalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"physical".as_ref(),
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

pub fn update_quantity_handler(ctx: Context<UpdatePhysicalQuantity>, qnt: u32) -> Result<()>{
    ctx.accounts.product.quantity = qnt;
    if qnt == 0{
        mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)?;
    };
    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePhysicalQuantityInternal<'info>{
    #[account(
        mut,
        seeds = [
            b"physical_product",
            vendor_listings.key().as_ref(),
            &[product.metadata.index]
        ],
        bump,
    )]
    pub product: Account<'info, PhysicalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"physical".as_ref(),
            &vendor_account.voter_id.to_le_bytes()
        ],
        bump
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    pub vendor_account: Account<'info, OrbitMarketAccount>,

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
        address = Pubkey::new(PHYSICAL_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}

pub fn update_quantity_internal_handler(ctx: Context<UpdatePhysicalQuantityInternal>, qnt: u32) -> Result<()>{
    ctx.accounts.product.quantity = qnt;
    if qnt == 0{
        mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)?;
    };
    Ok(())
}

#[derive(Accounts)]
pub struct IncrementPhysicalSoldInternal<'info>{
    #[account(
        mut
    )]
    pub product: Account<'info, PhysicalProduct>,

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
        address = Pubkey::new(PHYSICAL_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}

pub fn physical_increment_times_sold_handler(ctx: Context<IncrementPhysicalSoldInternal>) -> Result<()>{
    ctx.accounts.product.metadata.times_sold += 1;
    Ok(())
}

////////////////////////////////////////////
/// GENERAL

#[derive(Accounts)]
pub struct UpdatePhysicalProductField<'info>{
    #[account(
        mut,
        constraint = product.metadata.owner_catalog == vendor_account.voter_id
    )]
    pub product: Account<'info, PhysicalProduct>,

    pub vendor_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = vendor_account.wallet
    )]
    pub wallet: Signer<'info>,
}

pub fn physical_update_info_handler(ctx: Context<UpdatePhysicalProductField>, info: String) -> Result<()>{
    ctx.accounts.product.metadata.info = info;
    Ok(())
}

pub fn physical_update_price_handler(ctx: Context<UpdatePhysicalProductField>, price: u64) -> Result<()>{
    ctx.accounts.product.metadata.price = price;
    Ok(())
}

pub fn physical_update_delivery_estimate_handler(ctx: Context<UpdatePhysicalProductField>, delivery_estimate: u8) -> Result<()>{
    ctx.accounts.product.metadata.delivery_estimate = delivery_estimate;
    Ok(())
}

pub fn physical_update_media_handler(ctx: Context<UpdatePhysicalProductField>, link: String) -> Result<()>{
    ctx.accounts.product.metadata.media = link;
    Ok(())
}

pub fn mark_physical_searchable_handler(ctx:Context<UpdatePhysicalProductField>) -> Result<()>{
    ctx.accounts.product.metadata.search_indexed = true;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePhysicalProductListingField<'info>{
    #[account(
        mut,
        seeds = [
            b"physical_product",
            vendor_listings.key().as_ref(),
            &[product.metadata.index]
        ],
        bump,
    )]
    pub product: Account<'info, PhysicalProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            b"physical".as_ref(),
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

pub fn physical_mark_available_handler(ctx: Context<UpdatePhysicalProductListingField>) -> Result<()>{
    mark_prod_available_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)
}

pub fn physical_mark_unavailable_handler(ctx: Context<UpdatePhysicalProductListingField>) -> Result<()>{
    mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)
}