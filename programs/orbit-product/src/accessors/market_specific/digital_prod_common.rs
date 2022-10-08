use anchor_lang::{
    prelude::*,
    AccountsClose
};
use crate::{OrbitProduct, OrbitProductTrait, CommonProdUtils};
use crate::{DigitalProduct, DigitalFileTypes, ListingsStruct, RecentMarketListings};






#[derive(Accounts, CommonProdUtils)]
pub struct UpdateProductField<'info>{
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

pub fn set_file_type_handler(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
    ctx.accounts.digital_product.digital_file_type = file_type;   
    Ok(())
}