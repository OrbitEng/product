use crate::product_struct::OrbitProduct;
use anchor_lang::{
    prelude::*,
    Accounts
};

pub trait OrbitProductTrait<'a, 'b, T, U>
    where T: Accounts<'a>, U: Accounts<'b>
{
    fn list(ctx: Context<T>, prod: OrbitProduct)-> Result<()>;
    fn unlist(ctx: Context<U>)-> Result<()>;
}