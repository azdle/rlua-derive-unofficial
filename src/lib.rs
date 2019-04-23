#![recursion_limit="128"]
// needed even in 2018 because `proc_macro` is a built-in crate
extern crate proc_macro;

#[macro_use]
extern crate darling;

//use proc_macro::TokenStream;
use darling::ast;
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rlua))]
struct ToLuaInputReceiver {
    ident: syn::Ident,
    data: ast::Data<(), ToLuaFieldReceiver>,
    #[darling(default)]
    as_lua_array: bool,
}

impl ToTokens for ToLuaInputReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ToLuaInputReceiver {
            ref ident,
            ref data,
            as_lua_array,
        } = *self;

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let mut max_index = 0;
        let mut insert_fields = quote! {};
        fields.into_iter().enumerate().for_each(|(idx, field)| {
            if field.ident == None && as_lua_array {
                panic!("`as_lua_array` must not be used on unnamed structs");
            }

            let (default_key, ident) = field.ident.as_ref().map(|id| (quote!(stringify!(#id)), quote!(#id))).unwrap_or_else(|| {
                let idx = syn::Index{ index: idx as u32, span: proc_macro2::Span::call_site()};
                (quote!(#idx+1), quote!(#idx))
            });

            let insert_field = match (&field.key, &field.index) {
                (Some(key), None) => quote! {
                    t.raw_set(#key, self.#ident)?;
                },
                (None, Some(index)) => {
                    max_index = u64::max(max_index, *index);
                    quote! {
                        t.raw_set(#index, self.#ident)?;
                    }
                }
                (Some(_), Some(_)) => {
                    panic!("can't be both an index and a value");
                }
                (None, None) => quote! {
                    t.raw_set(#default_key, self.#ident)?;
                },
            };

            insert_fields.extend(insert_field);
        });

        // I feel like I read this was a thing that would let me have an 'array' table that
        // contains nil values, but I can't find that again, maybe I'm crazy?
        let set_max_index_if_needed = if max_index > 0 {
            quote! {
                t.raw_set("n", #max_index)?;
            }
        } else {
            quote! {}
        };

        tokens.extend(quote! {
            impl<'lua> ::rlua::ToLua<'lua> for #ident {
                fn to_lua(self, lua: ::rlua::Context<'lua>) -> ::rlua::Result<::rlua::Value<'lua>> {
                    let t = lua.create_table()?;

                    #insert_fields

                    #set_max_index_if_needed

                    return Ok(::rlua::Value::Table(t));
                }
            }
        });
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(rlua))]
struct ToLuaFieldReceiver {
    ident: Option<syn::Ident>,
    #[darling(default)]
    index: Option<u64>,
    #[darling(default)]
    key: Option<String>,
}

#[proc_macro_derive(ToLua, attributes(rlua))]
pub fn to_lua_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let receiver = ToLuaInputReceiver::from_derive_input(&ast).unwrap();
    let output = quote!(#receiver);
    output.into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rlua))]
struct FromLuaInputReceiver {
    ident: syn::Ident,
    data: ast::Data<(), FromLuaFieldReceiver>,
    #[darling(default)]
    as_lua_array: bool,
}

impl ToTokens for FromLuaInputReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FromLuaInputReceiver {
            ref ident,
            ref data,
            as_lua_array,
        } = *self;

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let mut max_index = 0;
        let mut extract_fields = quote! {};
        fields.into_iter().enumerate().for_each(|(idx, field)| {
            if field.ident == None && as_lua_array {
                panic!("`as_lua_array` must not be used on unnamed structs");
            }

            let (default_key, ident) = field.ident.as_ref().map(|id| (quote!(stringify!(#id)), quote!(#id))).unwrap_or_else(|| {
                let idx = syn::Index{ index: idx as u32, span: proc_macro2::Span::call_site()};
                (quote!(#idx+1), quote!(#idx))
            });

            let extract_field = match (&field.key, &field.index) {
                (Some(key), None) => quote! {
                    #ident: t.get(#key)?,
                },
                (None, Some(index)) => {
                    max_index = u64::max(max_index, *index);
                    quote! {
                        #ident: t.get(#index)?,
                    }
                }
                (Some(_), Some(_)) => {
                    panic!("can't be both an index and a value");
                }
                (None, None) => quote! {
                    #ident: t.get(#default_key)?,
                },
            };

            extract_fields.extend(extract_field);
        });

        tokens.extend(quote! {
            impl<'lua> ::rlua::FromLua<'lua> for #ident {
                fn from_lua(lua_value: ::rlua::Value<'lua>, lua: ::rlua::Context<'lua>) -> ::rlua::Result<Self> {
                    let t = rlua::Table::from_lua(lua_value, lua)?;

                    return Ok(Self{
                        #extract_fields
                    });
                }
            }
        });
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(rlua))]
struct FromLuaFieldReceiver {
    ident: Option<syn::Ident>,
    #[darling(default)]
    index: Option<u64>,
    #[darling(default)]
    key: Option<String>,
}

#[proc_macro_derive(FromLua, attributes(rlua))]
pub fn from_lua_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let receiver = FromLuaInputReceiver::from_derive_input(&ast).unwrap();
    let output = quote!(#receiver);
    output.into()
}
