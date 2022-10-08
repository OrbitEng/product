extern crate proc_macro;
use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(CommonProdUtils)]
pub fn derive_common_prod_utils(item: TokenStream) -> TokenStream{
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;

    let mut field_name: Option<Ident> = None;

    if let syn::Data::Struct(prod_ctx) = ast.data{
        if let syn::Fields::Named(ctx_fields) = prod_ctx.fields{
            field_name = ctx_fields.named.first().unwrap().ident.to_owned();
        }
    };

    if field_name.is_none(){
        panic!()
    }

    let expanded = quote!{
        pub fn update_price_handler(ctx: Context< #name >, price: u64) -> Result<()>{
            let mut mut_data = ctx.accounts.#field_name.try_borrow_mut_data()?;
            let price_vec = price.to_le_bytes();
            for i in 0..8{
                mut_data[117+i] = price_vec[i];
            }
            ctx.accounts.#field_name.exit(&crate::ID)
        }
        
        pub fn update_currency_handler(ctx: Context< #name >, currency: Pubkey) -> Result<()>{
            let mut mut_data = ctx.accounts.#field_name.try_borrow_mut_data()?;
            for (ind, byte_val) in currency.to_bytes().iter().enumerate(){
                mut_data[85+ind] = *byte_val;
            };
            ctx.accounts.#field_name.exit(&crate::ID)
        }
        
        pub fn update_media_handler(ctx: Context< #name >, link: String) -> Result<()>{
            let mut mut_data = ctx.accounts.#field_name.try_borrow_mut_data()?;
            for (ind, byte_val) in link.as_bytes().iter().enumerate(){
                mut_data[127+ind] = *byte_val;
            };
            ctx.accounts.#field_name.exit(&crate::ID)
        }
        
        pub fn update_info_handler(ctx: Context< #name >, info: String) -> Result<()>{
            let mut mut_data = ctx.accounts.#field_name.try_borrow_mut_data()?;
            for (ind, byte_val) in info.as_bytes().iter().enumerate(){
                mut_data[9+ind] = *byte_val;
            };
            ctx.accounts.#field_name.exit(&crate::ID)
        }

        pub fn mark_available_handler(ctx: Context< #name >) -> Result<()>{
            mark_prod_available_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.try_borrow_data()?[84])
        }

        pub fn mark_unavailable_handler(ctx: Context< #name >) -> Result<()>{
            mark_prod_unavailable_handler(&mut ctx.accounts.vendor_listings, ctx.accounts.product.try_borrow_data()?[84])
        }
    };

    TokenStream::from(expanded)
}