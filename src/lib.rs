#![recursion_limit="1024"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Data, Fields};

#[proc_macro_derive(ToLua)]
pub fn to_lua_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_to_lua(&ast)
}

fn impl_to_lua(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    if let Data::Struct(s) = data {
        let fields = if let Fields::Named(f) = &s.fields {
            f
        } else {
            panic!("named fields only");
        };

        let mut fields_code = quote!{};
        for item in fields.named.iter() {
            if let Some(ref ident) = item.ident {
                fields_code.extend(quote!{
                    t.raw_set(stringify!(#ident), self.#ident)?;
                });
            }
        }

        let gen = quote!{
            #[automatically_derived]
            impl<'lua> ::rlua::ToLua<'lua> for #name {
                fn to_lua(self, lua: ::rlua::Context<'lua>) -> ::rlua::Result<::rlua::Value<'lua>> {
                    use ::rlua::Table;
                    let t = lua.create_table()?;

                    #fields_code

                    Ok(::rlua::Value::Table(t))
                }
            }
        };
        gen.into()
    } else {
        panic!("must be used on a struct");
    }
}
