use crate::attrs::*;
use crate::proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn;
use syn::{Data, DataEnum, Fields, FieldsNamed, FieldsUnnamed, Generics, Index};

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
            enum_to_lua(name, e, attrs, &ast.generics)
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

fn enum_to_lua(name: &Ident, e: &DataEnum, attrs: EnumContainerAttrs, generics: &Generics) -> TokenStream {
    use inflector::Inflector; // trait & impl for changes to capitalization in strings

    let mut match_arms = quote! {};

    let generic_types = {
        let mut g = quote!{};
        generics.type_params().for_each(|type_param| {
            let t = &type_param.ident;
            g.extend(quote!{#t,})
        });
        g
    };

    let where_generics = {
        let mut g = quote!{where};
        generics.type_params().for_each(|type_param| {
            let t = &type_param.ident;
            g.extend(quote!{#t: rlua::ToLua<'lua> + Send, })
        });
        g
    };

    for v in &e.variants {
        let ident = &v.ident;

        let content_key = if let Some(content) = &attrs.content {
            content.clone()
        } else {
            ident.to_string().to_snake_case()
        };

        let set_tag = if let Some(tag) = &attrs.tag {
            let lua_ident = ident.to_string().to_snake_case();
            quote! {
                t.set(#tag, #lua_ident)?;
            }
        } else {
            quote! {}
        };

        match_arms.extend(quote! {
            #name::#ident(v) => {
                t.set(#content_key, v)?;
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
        impl<'lua, #generic_types> ::rlua::ToLua<'lua> for #name<#generic_types> #where_generics {
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
