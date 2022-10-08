use anchor_lang::{
    prelude::*,
    AccountsClose
};
use product::{product_struct::OrbitProduct, product_trait::OrbitProductTrait, CommonProdUtils};
use crate::{CommissionProduct, ListingsStruct, RecentMarketListings};


#[derive(Accounts, CommonProdUtils)]
pub struct UpdateProductField<'info>{

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
    pub seller_wallet: Signer<'info>,
}