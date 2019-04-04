use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;
use syn::{Data, Index, Fields, FieldsNamed, FieldsUnnamed};

pub fn impl_to_lua(ast: &syn::DeriveInput) -> TokenStream {
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

    for (i, item) in fields.unnamed.iter().enumerate() {
        if let None = item.ident {
            if i > std::u32::MAX as usize {
                panic!("structs with more than {} unnamed fields can not be supported", std::u32::MAX);
            }
            let idx = Index{ index: i as u32, span: Span::call_site()};
            fields_code.extend(quote!{
                t.raw_set(#idx + 1, self.#idx)?;
            });
        } else {
            panic!("struct with unnamed fields has named fields?");
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
