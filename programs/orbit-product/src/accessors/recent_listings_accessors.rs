use anchor_lang::{prelude::*, Discriminator};

use crate::{
    RecentMarketListings,
    PhysicalProduct,
    DigitalProduct,
    CommissionProduct
};

#[derive(Accounts)]
pub struct InitMarketRecentListings<'info>{
    #[account(
        init,
        space = 1000,
        seeds = [
            b"recent_listings",
            (PhysicalProduct::discriminator()).as_ref()
        ],
        bump,
        payer = payer
    )]
    pub physical_recent_listings: Account<'info, RecentMarketListings>,

    #[account(
        init,
        space = 1000,
        seeds = [
            b"recent_listings",
            (DigitalProduct::discriminator()).as_ref()
        ],
        bump,
        payer = payer
    )]
    pub digital_recent_listings: Account<'info, RecentMarketListings>,

    #[account(
        init,
        space = 1000,
        seeds = [
            b"recent_listings",
            (CommissionProduct::discriminator()).as_ref()
        ],
        bump,
        payer = payer
    )]
    pub commission_recent_listings: Account<'info, RecentMarketListings>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>
}

// #[derive(Accounts)]
// pub struct EditModCatalog<'info>{
//     #[account(
//         mut,
//         seeds = [
//             b"recent_listings",
//             (product.try_borrow_data()?)[0..8].as_ref()
//         ],
//         bump,
//     )]
//     pub catalog: Box<Account<'info, RecentMarketListings>>,

//     #[account(
//         constraint = *product.owner == crate::ID,
//         constraint = {
//             let dat = &(product.try_borrow_data()?)[0..8];
//             (dat == PhysicalProduct::discriminator()) ||
//             (dat == DigitalProduct::discriminator()) ||
//             (dat == CommissionProduct::discriminator())
//         }
//     )]
//     /// CHECK: base checks done
//     pub product: AccountInfo<'info>
// }

pub fn init_recent_listings_handler(_ctx: Context<InitMarketRecentListings>) -> Result<()>{
    Ok(())
}


pub fn edit_recent_listings_handler(recent_catalog: &mut Account<RecentMarketListings>, new_prod: AccountInfo) -> Result<()>{
    let index = recent_catalog.index;
    recent_catalog.pubkeys[index as usize] = new_prod.key();
    recent_catalog.index = (index+1) % 25;
    Ok(())
}