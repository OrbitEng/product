extern crate self as product;

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
pub mod orbit_digital_market {
    use super::*;

    /// VENDOR LISTINGS UTILS
    pub fn init_vendor_listings(ctx: Context<CreateVendorListing>) -> Result<()>{
        init_vendor_listings_handler(ctx)
    }

    pub fn list_product(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
        list_product_handler(ctx, listings_index)
    }
    pub fn unlist_product(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
        unlist_product_handler(ctx, listings_index)
    }
    pub fn mark_prod_available(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
        mark_prod_available_handler(ctx, listings_index)
    }
    pub fn mark_prod_unavailable(ctx: Context<ModifyVendorListings>, listings_index: u8) -> Result<()>{
        mark_prod_unavailable_handler(ctx, listings_index)
    }
    
    /// OWNERSHIP TRANSFER
    pub fn transfer_vendor_listings_ownership(ctx: Context<TransferOwner>) -> Result<()>{
        transfer_vendor_listings_ownership_handler(ctx)
    }
    pub fn transfer_all_vendor_listings_ownership(ctx: Context<TransferAllOwner>) -> Result<()>{
        transfer_all_vendor_listings_ownership_handler(ctx)
    }

    pub fn vendor_force_change_ownership(ctx: Context<VendorForceChangeOwner>) -> Result<()>{
        vendor_force_change_ownership_handler(ctx)
    }
}