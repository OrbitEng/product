use anchor_lang::{
    prelude::*,
    AccountsClose
};
use orbit_catalog::{
    structs::{
        OrbitModCatalogStruct,
        OrbitVendorCatalog
    },
    cpi::{
        accounts::{
            EditModCatalog,
            ModifyVendorCatalog
        },
        edit_mod_catalog, list_product, unlist_product
    }, program::OrbitCatalog
};
use product::{product_struct::OrbitProduct, product_trait::OrbitProductTrait, CommonProdUtils};
use crate::{CommissionProduct, CommissionMarketErrors, program::OrbitCommissionMarket};

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
    pub vendor_catalog:Box<Account<'info, OrbitVendorCatalog>>,

    #[account(
        mut,
        address = vendor_catalog.catalog_owner
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
    pub recent_catalog: Box<Account<'info, OrbitModCatalogStruct>>,

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub market_auth: SystemAccount<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,

    pub commission_program: Program<'info, OrbitCommissionMarket>,
}

#[derive(Accounts)]
pub struct UnlistCommissionProduct<'info>{

    #[account(mut)]
    pub commission_product: Box<Account<'info, CommissionProduct>>,

    #[account(
        address = commission_product.metadata.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, OrbitVendorCatalog>>,

    #[account(
        mut,
        address = vendor_catalog.catalog_owner
    )]
    pub seller_wallet: Signer<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,
}

impl <'a, 'b> OrbitProductTrait<'a, 'b, ListCommissionProduct<'a>, UnlistCommissionProduct<'b>> for CommissionProduct{
    fn list(ctx: Context<ListCommissionProduct>, prod: OrbitProduct)-> Result<()> {
        if prod.owner_catalog != ctx.accounts.vendor_catalog.key() {
            return err!(CommissionMarketErrors::InvalidSellerForListing)
        }
        list_product(
            CpiContext::new(
                ctx.accounts.catalog_program.to_account_info(),
                ModifyVendorCatalog{
                    vendor_catalog: ctx.accounts.vendor_catalog.to_account_info(),
                    wallet: ctx.accounts.seller_wallet.to_account_info()
                }
            ), 
            prod.index.clone()
        ).expect("could not list product, index is taken");

        ctx.accounts.commission_product.metadata = prod;
        match ctx.bumps.get("market_auth"){
            Some(auth_bump) => edit_mod_catalog(
                CpiContext::new_with_signer(
                    ctx.accounts.catalog_program.to_account_info(),
                    EditModCatalog {
                        catalog: ctx.accounts.recent_catalog.to_account_info(),
                        product: ctx.accounts.commission_product.to_account_info(),
                        caller_auth: ctx.accounts.market_auth.to_account_info()
                    },
                    &[&[b"market_auth", &[*auth_bump]]])
            ),
            None => err!(CommissionMarketErrors::InvalidAuthBump)
        }
    }

    fn unlist(ctx: Context<UnlistCommissionProduct>)-> Result<()> {
        unlist_product(
            CpiContext::new(
                ctx.accounts.catalog_program.to_account_info(),
                ModifyVendorCatalog{
                    vendor_catalog: ctx.accounts.vendor_catalog.to_account_info(),
                    wallet: ctx.accounts.seller_wallet.to_account_info()
                }
            ),
            ctx.accounts.commission_product.metadata.index
        ).expect("could not update catalog");
        ctx.accounts.commission_product.close(ctx.accounts.seller_wallet.to_account_info())
    }
}

#[derive(Accounts, CommonProdUtils)]
pub struct UpdateProductField<'info>{

    #[account(mut)]
    pub commission_product: Box<Account<'info, CommissionProduct>>,

    #[account(
        address = commission_product.metadata.owner_catalog
    )]
    pub vendor_catalog:Box<Account<'info, OrbitVendorCatalog>>,

    #[account(
        mut,
        address = vendor_catalog.catalog_owner
    )]
    pub seller_wallet: Signer<'info>,
}