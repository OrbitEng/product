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
            ctx.accounts.#field_name.metadata.price = price;
            Ok(())
        }
        
        pub fn update_currency_handler(ctx: Context< #name >, currency: Pubkey) -> Result<()>{
            ctx.accounts.#field_name.metadata.currency = currency;
            Ok(())
        }
        
        pub fn update_media_handler(ctx: Context< #name >, link: String) -> Result<()>{
            ctx.accounts.#field_name.metadata.media = link;
            Ok(())
        }
        
        pub fn update_info_handler(ctx: Context< #name >, info: String) -> Result<()>{
            ctx.accounts.#field_name.metadata.info = info;
            Ok(())
        }
    };

    TokenStream::from(expanded)
}