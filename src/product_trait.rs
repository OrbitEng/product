use crate::product_struct::OrbitProduct;
use anchor_lang::{
    prelude::*,
    Accounts
};

pub trait OrbitProductTrait<T, U>
    where T: for<'a> Accounts<'a>, U: for<'b> Accounts<'b>
{
    fn list(ctx: Context<T>, prod: OrbitProduct)-> Result<()> ;
    fn unlist(ctx: Context<U>)-> Result<()> where for<'a> T: Accounts<'a>;
}