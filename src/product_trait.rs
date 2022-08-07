use anchor_lang::prelude::*;

pub trait OrbitProductTrait{
    fn list<T>(ctx: Context<T>)-> Result<()> where for<'a> T: Accounts<'a> ;
    fn unlist<'a, T: Accounts<'a>>(ctx: Context<T>) -> Result<()>;
}