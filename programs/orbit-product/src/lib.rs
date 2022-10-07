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
}