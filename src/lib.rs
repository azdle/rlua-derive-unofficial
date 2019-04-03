#![recursion_limit="1024"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn;
use syn::{Data, Fields, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(ToLua)]
pub fn to_lua_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_to_lua(&ast)
}

fn impl_to_lua(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    match data {
        Data::Struct(s) => {
            match s.fields {
                Fields::Named(ref fields) => struct_named_to_lua(fields, name),
                Fields::Unnamed(ref fields) => struct_unnamed_to_lua(fields, name),
                Fields::Unit => struct_unit_to_lua(name),
            }
        },
        Data::Enum(_e) => {
            panic!("enums not supported");
        }
        Data::Union(_) => {
            panic!("unions not supported");
        }
    }

}

fn struct_named_to_lua(fields: &FieldsNamed, name: &Ident) -> TokenStream {
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
}

fn struct_unnamed_to_lua(fields: &FieldsUnnamed, name: &Ident) -> TokenStream {
    let mut fields_code = quote!{};
    for item in fields.unnamed.iter() {
        if let Some(ref ident) = item.ident {
            println!("unnamed ident: {}", ident);
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
}

fn struct_unit_to_lua(name: &Ident) -> TokenStream {
    let gen = quote!{
        #[automatically_derived]
        impl<'lua> ::rlua::ToLua<'lua> for #name {
            fn to_lua(self, lua: ::rlua::Context<'lua>) -> ::rlua::Result<::rlua::Value<'lua>> {
                use ::rlua::Table;
                let t = lua.create_table()?;
                Ok(::rlua::Value::Table(t))
            }
        }
    };
    gen.into()
}
