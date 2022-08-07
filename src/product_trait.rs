use anchor_lang::prelude::*;

pub trait OrbitProductTrait{
    fn list<'a, T: Accounts<'a>>(ctx: Context<T>) -> Result<()>;
    fn unlist<'a, T: Accounts<'a>>(ctx: Context<T>) -> Result<()>;
}