use anchor_lang::{prelude::*, Accounts};

pub trait OrbitProductTrait{
    fn list<T>(ctx: Context<T>)-> Result<()> where for<'a> T: Accounts<'a>;
    // fn unlist<T: >(ctx: Context<T>) -> Result<()>;
}