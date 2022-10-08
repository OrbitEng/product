use std::borrow::Borrow;

use anchor_lang::{
    prelude::*,
    AccountsClose
};
use orbit_derive_product::CommonProdUtils;

use crate::{
    PhysicalProduct,
    DigitalProduct,
    CommissionProduct,
    ListingsStruct,
    OrbitProduct
};

/////////////////////////////////////////
/// COMMISSION

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
pub struct ListCommissionProduct<'info>{
    
    #[account(
        init,
        space = 200,
        payer = seller_wallet,
        seeds = [
            b"commission_product",
            vendor_catalog.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub commission_product: Box<Account<'info, CommissionProduct>>,

    #[account(
        address = prod_in.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
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

#[derive(Accounts)]
pub struct UnlistCommissionProduct<'info>{

    #[account(mut)]
    pub commission_product: Box<Account<'info, CommissionProduct>>,

    #[account(
        address = commission_product.metadata.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
    )]
    pub seller_wallet: Signer<'info>
}

impl CommissionProduct{
    fn list(ctx: Context<ListCommissionProduct>, prod: OrbitProduct)-> Result<()> {
        ctx.accounts.commission_product.metadata = prod;
        Ok(())    
    }

    fn unlist(ctx: Context<UnlistCommissionProduct>)-> Result<()> {
        
        ctx.accounts.commission_product.close(ctx.accounts.seller_wallet.to_account_info())
    }
}


/////////////////////////////////////////////////
/// DIGITAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
pub struct ListDigitalProduct<'info>{
    
    #[account(
        init,
        space = 200,
        payer = seller_wallet,
        seeds = [
            b"commission_product",
            vendor_catalog.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    #[account(
        address = prod_in.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
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

#[derive(Accounts)]
pub struct UnlistDigitalProduct<'info>{

    #[account(mut)]
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    #[account(
        address = digital_product.metadata.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
    )]
    pub seller_wallet: Signer<'info>,
}


impl DigitalProduct{
    fn list(ctx: Context<ListDigitalProduct>, prod: OrbitProduct)-> Result<()> {
        ctx.accounts.digital_product.metadata = prod;
        Ok(())
    }

    fn unlist(ctx: Context<UnlistDigitalProduct>)-> Result<()> {
        ctx.accounts.digital_product.close(ctx.accounts.seller_wallet.to_account_info())
    }
}

////////////////////////////////////////////////
/// PHYSICAL

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
pub struct ListPhysicalProduct<'info>{

    #[account(
        init,
        space = 300, // 106 + 8. leave room for adjustment during launch
        payer = seller_wallet,

        seeds = [
            b"physical_product",
            vendor_catalog.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub phys_product: Box<Account<'info, PhysicalProduct>>,

    #[account(
        address = phys_product.metadata.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
    )]
    pub seller_wallet: Signer<'info>,
    
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct UnlistPhysicalProduct<'info>{
    #[account(mut)]
    pub phys_product: Box<Account<'info, PhysicalProduct>>,

    #[account(
        address = phys_product.metadata.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
    )]
    pub seller_wallet: Signer<'info>
}


impl PhysicalProduct{
    fn list(ctx: Context<ListPhysicalProduct>, prod: OrbitProduct) -> Result<()>{

        ctx.accounts.phys_product.metadata = prod;
        ctx.accounts.phys_product.quantity = 0;
        Ok(())
    }

    fn unlist(ctx: Context<UnlistPhysicalProduct>) -> Result<()>{
        ctx.accounts.phys_product.close(ctx.accounts.seller_wallet.to_account_info())
    }
}

////////////////////////////////////////////
/// GENERAL


#[derive(Accounts, CommonProdUtils)]
pub struct UpdateProductField<'info>{
    #[account(
        mut,
        constraint = *product.owner == crate::ID,
    )]
    /// CHECK: we check the program owns it
    pub product: AccountInfo<'info>,

    #[account(
        constraint = product.try_borrow_data()?[52..84] == vendor_catalog.key().to_bytes()
    )]
    pub vendor_catalog:Box<Account<'info, ListingsStruct>>,

    #[account(
        mut,
        address = vendor_catalog.listings_owner
    )]
    pub seller_wallet: Signer<'info>,
}

pub fn update_quantity_handler(ctx: Context<UpdateProductField>, qnt: u32) -> Result<()>{
    for (i, j) in ctx.accounts.product.key().to_bytes().iter().enumerate(){

    }
    let mut phys_prod = Account::<PhysicalProduct>::try_from(&ctx.accounts.product)?;
    phys_prod.quantity = qnt;
    Ok(())
}