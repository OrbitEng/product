use anchor_lang::prelude::*;

pub mod accessors;
pub mod structs;
pub mod errors;

pub use accessors::*;
pub use structs::*;
pub use errors::*;

declare_id!("97yvrxpWXrsurDgiWiskJ4KcQhFZJF6SrLoUYA53bpBL");

#[program]
pub mod orbit_product {

    use super::*;

    /// INIT
    pub fn init_recent_listings(ctx: Context<InitMarketRecentListings>) -> Result<()>{
        init_recent_listings_handler(ctx)
    }

    /// VENDOR LISTINGS UTILS
    pub fn init_commissions_listings(ctx: Context<CreateCommissionsListing>) -> Result<()>{
        init_commissions_listings_handler(ctx)
    }

    pub fn init_digital_listings(ctx: Context<CreateDigitalListing>) -> Result<()>{
        init_digital_listings_handler(ctx)
    }

    pub fn init_physical_listings(ctx: Context<CreatePhysicalListings>) -> Result<()>{
        init_physical_listings_handler(ctx)
    }

    // PRODUCT UTILS

    pub fn list_commission_product(ctx: Context<ListCommissionProduct>, prod: OrbitProductStruct) -> Result<()>{
        CommissionProduct::list(ctx, prod)
    }
    
    pub fn list_digital_product(ctx: Context<ListDigitalProduct>, prod: OrbitProductStruct, file_type: DigitalFileTypes) -> Result<()>{
        DigitalProduct::list(ctx, prod, file_type)
    }

    pub fn list_physical_product(ctx: Context<ListPhysicalProduct>, prod: OrbitProductStruct, quantity: u32) -> Result<()>{
        PhysicalProduct::list(ctx, prod, quantity)
    }

    pub fn unlist_commission_product(ctx: Context<UnlistCommissionProduct>) -> Result<()>{
        CommissionProduct::unlist(ctx)
    }
    
    pub fn unlist_digital_product(ctx: Context<UnlistDigitalProduct>) -> Result<()>{
        DigitalProduct::unlist(ctx)
    }

    pub fn unlist_physical_product(ctx: Context<UnlistPhysicalProduct>) -> Result<()>{
        PhysicalProduct::unlist(ctx)
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
    pub fn set_media(ctx: Context<UpdateProductField>, link: String) -> Result<()>{
        update_media_handler(ctx, link)
    }
    pub fn update_delivery_estimate(ctx: Context<UpdateProductField>, delivery_estimate: u8) -> Result<()>{
        update_delivery_estimate_handler(ctx, delivery_estimate)
    }
    pub fn set_prod_info(ctx: Context<UpdateProductField>, info: String) -> Result<()>{
        update_info_handler(ctx, info)
    }

    /// PHYSICAL
    pub fn update_product_quantity(ctx: Context<UpdateProductField>, qnt: u32) -> Result<()>{
        update_quantity_handler(ctx, qnt)
    }

    pub fn update_product_quantity_internal(ctx: Context<UpdateProductFieldInternal>, qnt: u32) -> Result<()>{
        update_quantity_internal_handler(ctx, qnt)
    }

    /// DIGITAL
    
    pub fn set_file_type(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
        set_file_type_handler(ctx, file_type)
    }

    pub fn flush_listings(ctx: Context<FlushListings>) -> Result<()>{
        flush_listings_handler(ctx)
    }

}