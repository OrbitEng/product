use anchor_lang::{
    prelude::*
};
use market_accounts::OrbitMarketAccount;

use crate::{
    structs::CommissionProduct,
    structs::ListingsStruct,
    structs::OrbitProductStruct,
    structs::RecentMarketListings,
    structs::ListingsType,

    errors::ProductErrors,

    accessors::vendor_listings_accessors::list_product_handler,
    accessors::vendor_listings_accessors::mark_prod_available_handler,
    accessors::vendor_listings_accessors::mark_prod_unavailable_handler,
    accessors::recent_listings_accessors::edit_recent_listings_handler,
};
/////////////////////////////////////////
/// COMMISSION

#[derive(Accounts)]
#[instruction(prod_in: OrbitProductStruct)]
pub struct ListCommissionProduct<'info>{
    
    #[account(
        init,
        space = 250,
        payer = wallet,
        seeds = [
            b"commission_product",
            vendor_listings.key().as_ref(),
            &[prod_in.index]
        ],
        bump,
        constraint = prod_in.owner_catalog == vendor_account.voter_id
    )]
    pub prod: Account<'info, CommissionProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            (&(ListingsType::Commissions).try_to_vec()?).as_slice(),
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
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlistCommissionProduct<'info>{
    #[account(
        mut,
        constraint = prod.metadata.owner_catalog == vendor_account.voter_id,
        close = wallet
    )]
    pub prod: Account<'info, CommissionProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            (&(ListingsType::Commissions).try_to_vec()?).as_slice(),
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

impl CommissionProduct{
    pub fn list(ctx: Context<ListCommissionProduct>, prod: OrbitProductStruct)-> Result<()> {
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;
        ctx.accounts.prod.metadata = prod;

        if ctx.remaining_accounts.len() == 1{
            let addr = Pubkey::find_program_address(&[
                b"recent_listings".as_ref(),
                b"commission".as_ref()
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

    pub fn unlist(ctx: Context<UnlistCommissionProduct>) -> Result<()>{
        //https://github.com/coral-xyz/anchor/blob/master/lang/src/common.rs

        let listings_index = ctx.accounts.prod.metadata.index;

        let avail_ind = 1<<(listings_index%64);
        let outer_ind = listings_index/64;
        
        ctx.accounts.vendor_listings.address_available[outer_ind as usize] |= avail_ind;
        ctx.accounts.vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
        
        Ok(())
    }
}

////////////////////////////////////////////
/// GENERAL

#[derive(Accounts)]
pub struct UpdateCommissionProductField<'info>{
    #[account(
        mut,
        seeds = [
            b"commission_product",
            vendor_listings.key().as_ref(),
            &[product.metadata.index]
        ],
        bump,
    )]
    pub product: Account<'info, CommissionProduct>,

    #[account(
        mut,
        seeds = [
            b"vendor_listings",
            (&(ListingsType::Commissions).try_to_vec()?).as_slice(),
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

pub fn commission_update_info_handler(ctx: Context<UpdateCommissionProductField>, info: String) -> Result<()>{
    ctx.accounts.product.metadata.info = info;
    Ok(())
}

pub fn commission_update_price_handler(ctx: Context<UpdateCommissionProductField>, price: u64) -> Result<()>{
    ctx.accounts.product.metadata.price = price;
    Ok(())
}

pub fn commission_update_delivery_estimate_handler(ctx: Context<UpdateCommissionProductField>, delivery_estimate: u8) -> Result<()>{
    ctx.accounts.product.metadata.delivery_estimate = delivery_estimate;
    Ok(())
}

pub fn commission_update_media_handler(ctx: Context<UpdateCommissionProductField>, link: String) -> Result<()>{
    ctx.accounts.product.metadata.media = link;
    Ok(())
}

pub fn commission_mark_available_handler(ctx: Context<UpdateCommissionProductField>) -> Result<()>{
    mark_prod_available_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)
}

pub fn commission_mark_unavailable_handler(ctx: Context<UpdateCommissionProductField>) -> Result<()>{
    mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.metadata.index)
}