use anchor_lang::prelude::*;

pub use orbit_derive_product::CommonProdUtils;

pub mod accessors;
pub mod structs;
pub mod errors;

pub use accessors::*;
pub use structs::*;
pub use errors::*;

declare_id!("DpKqMhUHc6YDjzGmxEKGZK8MxpdtW9X6jmYZrJ9UZj4g");

#[program]
pub mod orbit_product {
    use super::*;

    /// VENDOR LISTINGS UTILS
    pub fn init_vendor_listings(ctx: Context<CreateVendorListing>, market_type: String) -> Result<()>{
        init_vendor_listings_handler(ctx, market_type)
    }

    pub fn list_commission_product(ctx: Context<ListCommissionProduct>, prod: OrbitProduct) -> Result<()>{
        CommissionProduct::list(ctx, prod)
    }
    
    pub fn list_digital_product(ctx: Context<ListDigitalProduct>, prod: OrbitProduct, file_type: DigitalFileTypes) -> Result<()>{
        DigitalProduct::list(ctx, prod, file_type)
    }

    pub fn list_physical_product(ctx: Context<ListPhysicalProduct>, prod: OrbitProduct, quantity: u32) -> Result<()>{
        PhysicalProduct::list(ctx, prod, quantity)
    }
    
    /// OWNERSHIP TRANSFER
    pub fn transfer_vendor_listings_ownership(ctx: Context<TransferOwner>) -> Result<()>{
        transfer_vendor_listings_ownership_handler(ctx)
    }
    pub fn transfer_all_vendor_listings_ownership(ctx: Context<TransferAllOwner>) -> Result<()>{
        transfer_all_vendor_listings_ownership_handler(ctx)
    }

    /// PRODUCT MODIFIERS
    
    pub fn mark_prod_available(ctx: Context<UpdateProductField>) -> Result<()>{
        mark_available_handler(ctx)
    }
    pub fn mark_prod_unavailable(ctx: Context<UpdateProductField>) -> Result<()>{
        mark_unavailable_handler(ctx)
    }
    
    pub fn update_product_price(ctx: Context<UpdateProductField>, price: u64) -> Result<()>{
        update_price_handler(ctx, price)
    }
    pub fn update_currency(ctx: Context<UpdateProductField>, currency: Pubkey) -> Result<()>{
        update_currency_handler(ctx, currency)
    }
    pub fn set_media(ctx: Context<UpdateProductField>, link: String) -> Result<()>{
        update_media_handler(ctx, link)
    }
    pub fn set_prod_info(ctx: Context<UpdateProductField>, info: String) -> Result<()>{
        update_info_handler(ctx, info)
    }

    /// PHYSICAL
    pub fn update_product_quantity(ctx: Context<UpdateProductField>, qnt: u32) -> Result<()>{
        update_quantity_handler(ctx, qnt)
    }

    /// DIGITAL
    
    pub fn set_file_type(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
        set_file_type_handler(ctx, file_type)
    }
}