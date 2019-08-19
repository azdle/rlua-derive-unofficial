use crate::attrs::*;
use crate::proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn;
use syn::{Data, DataEnum, Fields, FieldsNamed, FieldsUnnamed, Index};

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
        Data::Enum(e) => {
            let attrs = parse_enum_container_attrs(&ast.attrs);
            enum_to_lua(name, e, attrs)
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
    let gen = quote! {
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

fn enum_to_lua(name: &Ident, e: &DataEnum, attrs: EnumContainerAttrs) -> TokenStream {
    let mut match_arms = quote! {};

    for v in &e.variants {
        let ident = &v.ident;

        let content_key = if let Some(content) = &attrs.content {
            proc_macro2::Ident::new(&content, proc_macro2::Span::call_site())
        } else {
            ident.clone()
        };

        let set_tag = if let Some(tag) = &attrs.tag {
            quote! {
                t.set(#tag, stringify!(#ident).to_lowercase())?;
            }
        } else {
            quote! {}
        };

        // TODO: figure out proper case-conversion for lua key names
        match_arms.extend(quote! {
            #name::#ident(v) => {
                t.set(stringify!(#content_key).to_lowercase(), v)?;
                #set_tag
            },
        });
    }

    let match_statement = quote! {
                match self {
                    #match_arms
                }
    };

    let gen = quote! {
        #[automatically_derived]
        impl<'lua> ::rlua::ToLua<'lua> for #name {
            fn to_lua(self, lua: ::rlua::Context<'lua>) -> ::rlua::Result<::rlua::Value<'lua>> {
                use ::rlua::Table;
                let t = lua.create_table()?;

                #match_statement

                Ok(::rlua::Value::Table(t))
            }
        }
    };
    gen.into()
}
