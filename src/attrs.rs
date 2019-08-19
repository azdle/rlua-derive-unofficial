use proc_macro2::Span;
use syn;
use syn::Attribute;

#[derive(Debug)]
pub struct EnumContainerAttrs {
    pub tag: Option<String>,
    pub content: Option<String>,
}

pub fn parse_enum_container_attrs(attrs: &[Attribute]) -> EnumContainerAttrs {
    let mut parsed = EnumContainerAttrs {
        tag: None,
        content: None,
    };

    for attr in attrs {
        if attr.path == syn::Ident::new("rlua", Span::call_site()).into() {
            match attr.parse_meta() {
                Ok(syn::Meta::Path(_ident)) => {
                    panic!("path");
                }
                Ok(syn::Meta::List(meta_list)) => {
                    for meta in meta_list.nested.iter() {
                        match meta {
                            syn::NestedMeta::Meta(meta) => {
                                match meta {
                                    syn::Meta::Path(_ident) => {
                                        panic!("path unsupported");
                                    }
                                    syn::Meta::List(_meta_list) => {
                                        panic!("list unsupported");
                                    }
                                    syn::Meta::NameValue(ident) => {
                                        // TODO: Do I really need to build this ident (into path)?
                                        if ident.path
                                            == syn::Ident::new("tag", Span::call_site()).into()
                                        {
                                            if let syn::Lit::Str(s) = &ident.lit {
                                                assert!(
                                                    parsed.tag.replace(s.value().clone()).is_none(),
                                                    "more than one `tag` value specified"
                                                );
                                            } else {
                                                panic!("tag takes a string");
                                            }
                                        } else if ident.path
                                            == syn::Ident::new("content", Span::call_site()).into()
                                        {
                                            if let syn::Lit::Str(s) = &ident.lit {
                                                assert!(
                                                    parsed
                                                        .content
                                                        .replace(s.value().clone())
                                                        .is_none(),
                                                    "more than one `content` value specified"
                                                );
                                            } else {
                                                panic!("content takes a string");
                                            }
                                        } else {
                                            panic!("unknown key")
                                        }
                                    }
                                }
                            }
                            syn::NestedMeta::Lit(_) => {
                                panic!("lit unsuported here");
                            }
                        }
                    }
                }
                Ok(syn::Meta::NameValue(_)) => {
                    panic!("name value");
                }
                Err(_) => (),
            }
        }
    }

    parsed
}
