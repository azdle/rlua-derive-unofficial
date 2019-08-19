use crate::attrs::*;
use crate::proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn;
use syn::{Data, DataEnum, Fields, FieldsNamed, FieldsUnnamed, Index};

pub fn impl_from_lua(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    match data {
        Data::Struct(s) => {
            match s.fields {
                Fields::Named(ref fields) => struct_named_from_lua(fields, name),
                Fields::Unnamed(ref fields) => struct_unnamed_from_lua(fields, name),
                Fields::Unit => struct_unit_from_lua(name),
            }
        },
        Data::Enum(e) => {
            let attrs = parse_enum_container_attrs(&ast.attrs);
            enum_from_lua(name, e, attrs)
        }
        Data::Union(_) => {
            panic!("unions not supported");
        }
    }

}

fn struct_named_from_lua(fields: &FieldsNamed, name: &Ident) -> TokenStream {
    let mut fields_code = quote!{};
    for item in fields.named.iter() {
        if let Some(ref ident) = item.ident {
            fields_code.extend(quote!{
                #ident: t.raw_get(stringify!(#ident))?,
            });
        }
    }

    let gen = quote!{
        #[automatically_derived]
        impl<'lua> ::rlua::FromLua<'lua> for #name {
            fn from_lua(value: ::rlua::Value<'lua>, lua: ::rlua::Context<'lua>) -> ::rlua::Result<Self> {
                use ::rlua::Table;

                if let ::rlua::Value::Table(t) = value {
                    Ok(Self{
                        #fields_code
                    })
                } else {
                    Err(rlua::Error::FromLuaConversionError {
                        from: "something not a table",
                        to: stringify!(#name),
                        message: None,
                    })
                }
            }
        }
    };
    gen.into()
}

fn struct_unnamed_from_lua(fields: &FieldsUnnamed, name: &Ident) -> TokenStream {
    let mut fields_code = quote!{};
    for (i, item) in fields.unnamed.iter().enumerate() {
        if let None = item.ident {
            if i > std::u32::MAX as usize {
                panic!("structs with more than {} unnamed fields can not be supported", std::u32::MAX);
            }
            let idx = Index{ index: i as u32, span: Span::call_site()};
            fields_code.extend(quote!{
                t.raw_get(#idx + 1)?,
            });
        } else {
            panic!("struct with unnamed fields has named fields?");
        }
    }

    let gen = quote!{
        #[automatically_derived]
        impl<'lua> ::rlua::FromLua<'lua> for #name {
            fn from_lua(value: ::rlua::Value<'lua>, lua: ::rlua::Context<'lua>) -> ::rlua::Result<Self> {
                use ::rlua::Table;

                if let ::rlua::Value::Table(t) = value {
                    Ok(Self(
                        #fields_code
                    ))
                } else {
                    Err(rlua::Error::FromLuaConversionError {
                        from: "something not a table",
                        to: stringify!(#name),
                        message: None,
                    })
                }
            }
        }
    };
    gen.into()
}

fn struct_unit_from_lua(name: &Ident) -> TokenStream {
    let gen = quote!{
        #[automatically_derived]
        impl<'lua> ::rlua::FromLua<'lua> for #name {
            fn from_lua(value: ::rlua::Value<'lua>, lua: ::rlua::Context<'lua>) -> ::rlua::Result<Self> {
                use ::rlua::Table;

                if let ::rlua::Value::Table(t) = value {
                    Ok(Self)
                } else {
                    Err(rlua::Error::FromLuaConversionError {
                        from: "something not a table",
                        to: stringify!(#name),
                        message: None,
                    })
                }
            }
        }
    };
    gen.into()
}

fn enum_from_lua(name: &Ident, e: &DataEnum, attrs: EnumContainerAttrs) -> TokenStream {
    use inflector::Inflector; // trait & impl for changes to capitalization in strings

    let get_key_and_value = match (&attrs.tag, &attrs.content) {
        (Some(tag), Some(content)) => quote! {
            let lua_key: String = t.get(#tag)?;
            let lua_value: ::rlua::Value = t.get(#content)?;
        },
        (Some(tag), None) => quote! {
            let lua_key: String = t.get(#tag)?;
            let lua_value: ::rlua::Value = t.get(lua_key.as_str())?;
        },
        // TODO: possible to support untagged enums?
        (None, Some(_content)) => panic!("can't specify content without key"),
        (None, None) => quote! {
            let (lua_key, lua_value) : (String, ::rlua::Value) = t.pairs().nth(0)
                .ok_or_else(|| {
                    ::rlua::Error::FromLuaConversionError {
                        from: "table",
                        to: stringify!(#name),
                        message: Some("table was empty".to_string()),
                    }
                })??;
        },
    };

    let mut match_arms = quote! {};

    for v in &e.variants {
        let ident = &v.ident;

        let lua_variant_name = ident.to_string().to_snake_case();

        match_arms.extend(quote! {
            #lua_variant_name => {
                Ok(#name::#ident(::rlua::FromLua::from_lua(lua_value, lua)?))
            },
        });
    }

    let match_statement = quote! {
                match lua_key.as_str() {
                    #match_arms
                    unknown_key =>
                        Err(::rlua::Error::FromLuaConversionError {
                            from: "table",
                            to: stringify!(#name),
                            message: Some(format!("unknown variant: {}", unknown_key)),
                        }),
                }
    };

    let gen = quote! {
        #[automatically_derived]
        impl<'lua> ::rlua::FromLua<'lua> for #name {
            fn from_lua(value: ::rlua::Value<'lua>, lua: ::rlua::Context<'lua>) -> ::rlua::Result<Self> {
                let t: ::rlua::Table = ::rlua::FromLua::from_lua(value, lua)?;

                #get_key_and_value

                #match_statement
            }
        }
    };
    gen.into()
}
