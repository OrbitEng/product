use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use orbit_derive_product::CommonProdUtils;

use crate::{
    PhysicalProduct,
    DigitalProduct,
    CommissionProduct,
    ListingsStruct,
    OrbitProduct,
    RecentMarketListings,
    DigitalFileTypes,

    list_product_handler,
    unlist_product_handler,
    mark_prod_available_handler,
    mark_prod_unavailable_handler,
};

/////////////////////////////////////////
/// COMMISSION

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
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

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub market_auth: SystemAccount<'info>
}

impl CommissionProduct{
    pub fn list(ctx: Context<ListCommissionProduct>, prod: OrbitProduct)-> Result<()> {
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;
        ctx.accounts.commission_product.metadata = prod;
        Ok(())    
    }
}


/////////////////////////////////////////////////
/// DIGITAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
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

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            b"recent_catalog"
        ],
        bump
    )]
    pub recent_catalog: Box<Account<'info, RecentMarketListings>>,
}


impl DigitalProduct{
    pub fn list(ctx: Context<ListDigitalProduct>, prod: OrbitProduct, file_type: DigitalFileTypes)-> Result<()> {
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;

        ctx.accounts.digital_product.metadata = prod;
        ctx.accounts.digital_product.digital_file_type = file_type;
        Ok(())
    }
}

////////////////////////////////////////////////
/// PHYSICAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
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


impl PhysicalProduct{
    pub fn list(ctx: Context<ListPhysicalProduct>, prod: OrbitProduct, quantity: u32) -> Result<()>{
        list_product_handler(&mut ctx.accounts.vendor_listings, prod.index)?;

        ctx.accounts.phys_product.metadata = prod;
        ctx.accounts.phys_product.quantity = quantity;
        Ok(())
    }
}

////////////////////////////////////////////
/// GENERAL

#[derive(Accounts)]
pub struct UnlistProduct<'info>{
    #[account(
        mut,
        constraint = *prod.owner == crate::ID
    )]
    pub prod: AccountInfo<'info>,

    #[account(
        constraint = prod.try_borrow_data()?[52..84] == vendor_listings.key().to_bytes()
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>
}

pub fn unlist(ctx: Context<UnlistProduct>) -> Result<()>{
    //https://github.com/coral-xyz/anchor/blob/master/lang/src/common.rs

    unlist_product_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.prod.try_borrow_data()?[84])?;

    let info = &ctx.accounts.prod;
    let sol_destination = ctx.accounts.seller_wallet.to_account_info();
    let dest_starting_lamports = sol_destination.lamports();
    **sol_destination.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(info.lamports()).unwrap();
    **info.lamports.borrow_mut() = 0;

    info.assign(&system_program::ID);
    info.realloc(0, false).map_err(Into::into)
}


#[derive(Accounts, CommonProdUtils)]
pub struct UpdateProductField<'info>{
    #[account(
        mut,
        constraint = *product.owner == crate::ID,
        constraint = product.data_len() == 250
    )]
    /// CHECK: we check the program owns it
    pub product: AccountInfo<'info>,

    #[account(
        constraint = product.try_borrow_data()?[52..84] == vendor_listings.key().to_bytes()
    )]
    pub vendor_listings: Account<'info, ListingsStruct>,

    #[account(
        mut,
        address = vendor_listings.listings_owner
    )]
    pub seller_wallet: Signer<'info>,
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

/////////////////////
/// DIGITAL ONLY

pub fn set_file_type_handler(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
    let mut digital_prod = Account::<DigitalProduct>::try_from(&ctx.accounts.product)?;
    digital_prod.digital_file_type = file_type;
    digital_prod.exit(&crate::ID)
}