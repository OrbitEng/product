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
    PhysicalProduct,
    DigitalProduct,
    CommissionProduct,
    ListingsStruct,
    OrbitProductStruct,
    RecentMarketListings,
    DigitalFileTypes,

    list_product_handler,
    mark_prod_available_handler,
    mark_prod_unavailable_handler, ProductErrors, edit_recent_listings_handler,
};

/////////////////////////////////////////
/// COMMISSION

#[derive(Accounts)]
#[instruction(prod_in: OrbitProductStruct)]
pub struct ListCommissionProduct<'info>{
    
    #[account(
        init,
        space = 250,
        payer = seller_wallet,
        seeds = [
            b"commission_product",
            vendor_listings.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub commission_product: Account<'info, CommissionProduct>,

    #[account(mut)]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct UnlistCommissionProduct<'info>{
    #[account(
        mut,
        close = seller_wallet
    )]
    pub prod: Account<'info, CommissionProduct>,

    #[account(
        mut,
        address = prod.metadata.owner_catalog
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>
}

impl CommissionProduct{
    pub fn list(ctx: Context<ListCommissionProduct>, prod: OrbitProductStruct)-> Result<()> {
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;
        ctx.accounts.commission_product.metadata = prod;

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
            edit_recent_listings_handler(recent_catalog, ctx.accounts.commission_product.to_account_info())?;
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


/////////////////////////////////////////////////
/// DIGITAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProductStruct)]
pub struct ListDigitalProduct<'info>{
    
    #[account(
        init,
        space = 250,
        payer = seller_wallet,
        seeds = [
            b"digital_product",
            vendor_listings.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub digital_product: Account<'info, DigitalProduct>,

    #[account(mut)]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}



#[derive(Accounts)]
pub struct UnlistDigitalProduct<'info>{
    #[account(
        mut,
        close = seller_wallet
    )]
    /// CHECK: we do owner check
    pub prod: Account<'info, DigitalProduct>,

    #[account(
        mut,
        address = prod.metadata.owner_catalog
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>
}


impl DigitalProduct{
    pub fn list(ctx: Context<ListDigitalProduct>, prod: OrbitProductStruct, file_type: DigitalFileTypes)-> Result<()> {
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;

        ctx.accounts.digital_product.metadata = prod;
        ctx.accounts.digital_product.digital_file_type = file_type;

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
            edit_recent_listings_handler(recent_catalog, ctx.accounts.digital_product.to_account_info())?;
            recent_catalog.exit(&crate::ID)?;
        }
        Ok(())
    }

    pub fn unlist(ctx: Context<UnlistDigitalProduct>) -> Result<()>{
        //https://github.com/coral-xyz/anchor/blob/master/lang/src/common.rs

        let listings_index = ctx.accounts.prod.metadata.index;

        let avail_ind = 1<<(listings_index%64);
        let outer_ind = listings_index/64;
        
        ctx.accounts.vendor_listings.address_available[outer_ind as usize] |= avail_ind;
        ctx.accounts.vendor_listings.product_available[outer_ind as usize] &= !(avail_ind as u64);
        
        Ok(())
    }
}

////////////////////////////////////////////////
/// PHYSICAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProductStruct)]
pub struct ListPhysicalProduct<'info>{

    #[account(
        init,
        space = 250, // 106 + 8. leave room for adjustment during launch
        payer = seller_wallet,

        seeds = [
            b"physical_product",
            vendor_listings.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub phys_product: Account<'info, PhysicalProduct>,

    #[account(mut)]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>,
    
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UnlistPhysicalProduct<'info>{
    #[account(
        mut,
        close = seller_wallet
    )]
    /// CHECK: we do owner check
    pub prod: Account<'info, PhysicalProduct>,

    #[account(
        mut,
        address = prod.metadata.owner_catalog
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>
}

impl PhysicalProduct{
    pub fn list(ctx: Context<ListPhysicalProduct>, prod: OrbitProductStruct, quantity: u32) -> Result<()>{
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;

        ctx.accounts.phys_product.metadata = prod;
        ctx.accounts.phys_product.quantity = quantity;

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
            edit_recent_listings_handler(recent_catalog, ctx.accounts.phys_product.to_account_info())?;
            recent_catalog.exit(&crate::ID)?;
        }
        Ok(())
    }

    
    pub fn unlist(ctx: Context<UnlistPhysicalProduct>) -> Result<()>{
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
    pub seller_wallet: Signer<'info>,
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

///////////////////////
/// PHYS ONLY

pub fn update_quantity_handler(ctx: Context<UpdateProductField>, qnt: u32) -> Result<()>{
    let mut phys_prod = Account::<PhysicalProduct>::try_from(&ctx.accounts.product)?;
    phys_prod.quantity = qnt;
    if qnt == 0{
        mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, phys_prod.metadata.index)?;
    }
    phys_prod.exit(&crate::ID)
}

pub fn update_quantity_internal_handler(ctx: Context<UpdateProductFieldInternal>, qnt: u32) -> Result<()>{
    let mut phys_prod = Account::<PhysicalProduct>::try_from(&ctx.accounts.product)?;
    phys_prod.quantity = qnt;
    if qnt == 0{
        mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, phys_prod.metadata.index)?;
    }
    phys_prod.exit(&crate::ID)
}

/////////////////////
/// DIGITAL ONLY

pub fn set_file_type_handler(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
    let mut digital_prod = Account::<DigitalProduct>::try_from(&ctx.accounts.product)?;
    digital_prod.digital_file_type = file_type;
    digital_prod.exit(&crate::ID)
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
    pub seller_wallet: Signer<'info>
}

pub fn flush_listings_handler(ctx: Context<FlushListings>) -> Result<()>{
    ctx.accounts.vendor_listings.address_available = [u64::MAX,u64::MAX,u64::MAX,u64::MAX];
    Ok(())
}