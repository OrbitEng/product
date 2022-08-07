use anchor_lang::{prelude::*, Accounts};
use crate::product_struct::OrbitProduct;

pub trait OrbitProductTrait{
    fn list<T>(ctx: Context<T>, prod: OrbitProduct)-> Result<()> where for<'a> T: Accounts<'a>;
    fn unlist<T>(ctx: Context<T>)-> Result<()> where for<'a> T: Accounts<'a>;
}