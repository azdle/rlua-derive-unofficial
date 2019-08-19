#![recursion_limit="1024"]

extern crate proc_macro;

mod attrs;
mod from_lua;
mod to_lua;

use crate::proc_macro::TokenStream;
use syn;

#[proc_macro_derive(FromLua, attributes(rlua))]
pub fn from_lua_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    from_lua::impl_from_lua(&ast)
}

#[proc_macro_derive(ToLua, attributes(rlua))]
pub fn to_lua_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    to_lua::impl_to_lua(&ast)
}
