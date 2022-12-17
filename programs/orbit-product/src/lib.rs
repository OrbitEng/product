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
        list_commission_handler(ctx, prod)
    }
    pub fn unlist_commission_product(ctx: Context<UnlistCommissionProduct>) -> Result<()>{
        unlist_commission_handler(ctx)
    }
    
    pub fn list_digital_product(ctx: Context<ListDigitalProduct>, prod: OrbitProductStruct, file_type: DigitalFileTypes) -> Result<()>{
        list_digital_handler(ctx, prod, file_type)
    }
    pub fn unlist_digital_product(ctx: Context<UnlistDigitalProduct>) -> Result<()>{
        unlist_digital_handler(ctx)
    }
    
    pub fn list_physical_product(ctx: Context<ListPhysicalProduct>, prod: OrbitProductStruct, quantity: u32) -> Result<()>{
        list_physical_handler(ctx, prod, quantity)
    }
    pub fn unlist_physical_product(ctx: Context<UnlistPhysicalProduct>) -> Result<()>{
        unlist_physical_handler(ctx)
    }
    
    /// OWNERSHIP TRANSFER
    pub fn transfer_vendor_listings_ownership(ctx: Context<TransferOwner>) -> Result<()>{
        transfer_vendor_listings_ownership_handler(ctx)
    }
    pub fn transfer_all_vendor_listings_ownership(ctx: Context<TransferAllOwner>) -> Result<()>{
        transfer_all_vendor_listings_ownership_handler(ctx)
    }

    /// PHYSICAL
    pub fn update_product_quantity(ctx: Context<UpdatePhysicalQuantity>, qnt: u32) -> Result<()>{
        update_quantity_handler(ctx, qnt)
    }

    pub fn update_product_quantity_internal(ctx: Context<UpdatePhysicalQuantityInternal>, qnt: u32) -> Result<()>{
        update_quantity_internal_handler(ctx, qnt)
    }

    pub fn physical_increment_times_sold(ctx: Context<UpdatePhysicalQuantityInternal>) -> Result<()>{
        physical_increment_times_sold_handler(ctx)
    }
        //
        pub fn physical_update_info(ctx: Context<UpdatePhysicalProductField>, info: String) -> Result<()>{
            physical_update_info_handler(ctx, info)
        }
        pub fn physical_update_price(ctx: Context<UpdatePhysicalProductField>, price: u64) -> Result<()>{
            physical_update_price_handler(ctx, price)
        }
        pub fn physical_update_delivery_estimate(ctx: Context<UpdatePhysicalProductField>, delivery_estimate: u8) -> Result<()>{
            physical_update_delivery_estimate_handler(ctx, delivery_estimate)
        }
        pub fn physical_update_media(ctx: Context<UpdatePhysicalProductField>, link: String) -> Result<()>{
            physical_update_media_handler(ctx, link)
        }
        pub fn physical_mark_available(ctx: Context<UpdatePhysicalProductField>) -> Result<()>{
            physical_mark_available_handler(ctx)
        }
        pub fn physical_mark_unavailable(ctx: Context<UpdatePhysicalProductField>) -> Result<()>{
            physical_mark_unavailable_handler(ctx)
        }

    /// DIGITAL
    
    pub fn set_file_type(ctx: Context<SetFileType>, file_type: DigitalFileTypes) -> Result<()>{
        set_file_type_handler(ctx, file_type)
    }
    
    pub fn digital_increment_times_sold(ctx: Context<UpdateDigitalQuantityInternal>) -> Result<()>{
        digital_increment_times_sold_handler(ctx)
    }
        //
        pub fn digital_update_info(ctx: Context<UpdateDigitalProductField>, info: String) -> Result<()>{
            digital_update_info_handler(ctx, info)
        }
        pub fn digital_update_price(ctx: Context<UpdateDigitalProductField>, price: u64) -> Result<()>{
            digital_update_price_handler(ctx, price)
        }
        pub fn digital_update_delivery_estimate(ctx: Context<UpdateDigitalProductField>, delivery_estimate: u8) -> Result<()>{
            digital_update_delivery_estimate_handler(ctx, delivery_estimate)
        }
        pub fn digital_update_media(ctx: Context<UpdateDigitalProductField>, link: String) -> Result<()>{
            digital_update_media_handler(ctx, link)
        }
        pub fn digital_mark_available(ctx: Context<UpdateDigitalProductField>) -> Result<()>{
            digital_mark_available_handler(ctx)
        }
        pub fn digital_mark_unavailable(ctx: Context<UpdateDigitalProductField>) -> Result<()>{
            digital_mark_unavailable_handler(ctx)
        }

    /// COMMISSIONS
    
    pub fn commission_increment_times_sold(ctx: Context<UpdateCommissionQuantityInternal>) -> Result<()>{
        commission_increment_times_sold_handler(ctx)
    }

        pub fn commission_update_info(ctx: Context<UpdateCommissionProductField>, info: String) -> Result<()>{
            commission_update_info_handler(ctx, info)
        }
        pub fn commission_update_price(ctx: Context<UpdateCommissionProductField>, price: u64) -> Result<()>{
            commission_update_price_handler(ctx, price)
        }
        pub fn commission_update_delivery_estimate(ctx: Context<UpdateCommissionProductField>, delivery_estimate: u8) -> Result<()>{
            commission_update_delivery_estimate_handler(ctx, delivery_estimate)
        }
        pub fn commission_update_media(ctx: Context<UpdateCommissionProductField>, link: String) -> Result<()>{
            commission_update_media_handler(ctx, link)
        }
        pub fn commission_mark_available(ctx: Context<UpdateCommissionProductField>) -> Result<()>{
            commission_mark_available_handler(ctx)
        }
        pub fn commission_mark_unavailable(ctx: Context<UpdateCommissionProductField>) -> Result<()>{
            commission_mark_unavailable_handler(ctx)
        }

    /// FLUSH LISTINGS
    pub fn flush_listings(ctx: Context<FlushListings>) -> Result<()>{
        flush_listings_handler(ctx)
    }
}