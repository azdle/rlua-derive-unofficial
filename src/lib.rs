//! This crate provides derive macros that allow you to derive `ToLua` and `FromLua` for any(*)
//! struct or enum consisting of types that implement `ToLua` or `FromLua`.
//!
//! *Note: This crate is incomplete there are some limitations currently, but these are considered
//! bugs and should be fixed. PRs welcome.
//!
//! # Examples
//!
//! The first example shows that basic usage of this library on a struct.
//!
//! ```rust
//! use rlua::Lua;
//! use rlua_derive_unofficial::{FromLua, ToLua};
//!
//! #[derive(FromLua, ToLua)]
//! struct SimpleStruct {
//!     message: String,
//!     count: u64,
//! }
//!
//! let simp = SimpleStruct{ message: "foo".to_string(), count: 37 };
//!
//! let lua = Lua::new();
//! let ret = lua.context(|l| {
//!     l.globals().set("simp", simp).unwrap();
//!     l.load(
//!         r#"
//!         assert(type(simp) == "table")
//!         assert(simp.message == "foo")
//!         assert(simp.count == 37)
//!
//!         return { message = "bar", count = 42 }
//!     "#,
//!     )
//!     .eval::<SimpleStruct>()
//!     .unwrap()
//! });
//!
//! assert!(ret.message == "bar");
//! assert!(ret.count == 42);
//! ```
//!
//! This example show the use on a basic Enum
//!
//! ```rust
//! use rlua::Lua;
//! use rlua_derive_unofficial::{FromLua, ToLua};
//!
//! # #[derive(PartialEq)]
//! #[derive(FromLua, ToLua)]
//! enum SimpleEnum {
//!     Message(String),
//!     Count(u64),
//! }
//!
//! let simp = SimpleEnum::Message("foo".to_string());
//!
//! let lua = Lua::new();
//! let ret = lua.context(|l| {
//!     l.globals().set("simp", simp).unwrap();
//!     l.load(
//!         r#"
//!         assert(type(simp) == "table")
//!         assert(simp.message == "foo")
//!
//!         return { count = 42 }
//!     "#,
//!     )
//!     .eval::<SimpleEnum>()
//!     .unwrap()
//! });
//!
//! assert!(ret == SimpleEnum::Count(42));
//! ```
//!
//! # Attribute Modifiers
//!
//! There are a few attributes that can be applied to adjust the Lua representation. These
//! modifiers are loosely based on those provided by [Serde](https://serde.rs/attributes.html).
//!
//! For reference these are the default representations:
//!
//! # Struct
//!
//! ```rust
//! # use rlua::Lua;
//! # use rlua_derive_unofficial::{FromLua, ToLua};
//! # #[derive(FromLua, ToLua)]
//! struct SimpleStruct {
//!     message: String,
//!     count: u64,
//! }
//! ```
//!
//! ```lua
//! { message = "foo", count = 37 }
//! ```

//! # Enum
//!
//! ```rust
//! # use rlua::Lua;
//! # use rlua_derive_unofficial::{FromLua, ToLua};
//! # #[derive(FromLua, ToLua)]
//! enum SimpleEnum {
//!     Message(String),
//!     Count(u64),
//! }
//! ```
//!
//! ```lua
//! { message = "foo" }
//! ```
//!
//! ## Container Attributes
//!
//! ### Enums
//!
//! #### `tag`
//!
//! `tag` allows you to specify a table key under which to store a string representation of the
//! enum variant name.
//!
//! ```rust
//! # use rlua::Lua;
//! # use rlua_derive_unofficial::{FromLua, ToLua};
//! # #[derive(FromLua, ToLua)]
//! #[rlua(tag = "type")]
//! enum SimpleEnum {
//!     Message(String),
//!     Count(u64),
//! }
//! ```
//!
//! ```lua
//! { type = "message", message = "foo" }
//! ```
//!
//! #### `content`
//!
//! `content` allows you to change the key under which the content of the enum will be stored. This
//! option will likey be used in conjunction with `tag`.
//!
//! ```rust
//! # use rlua::Lua;
//! # use rlua_derive_unofficial::{FromLua, ToLua};
//! # #[derive(FromLua, ToLua)]
//! #[rlua(tag = "type", content = "val")]
//! enum SimpleEnum {
//!     Message(String),
//!     Count(u64),
//! }
//! ```
//!
//! ```lua
//! { type = "message", val = "foo" }
//! ```
//!
//! #### `untagged`
//!
//! `untagged` allows you to pass the value contained in the enum directly.
//!
//! ```rust
//! # use rlua::Lua;
//! # use rlua_derive_unofficial::{FromLua, ToLua};
//! # #[derive(FromLua, ToLua)]
//! #[rlua(tag = "type", content = "val")]
//! enum SimpleEnum {
//!     Message(String),
//!     Count(u64),
//! }
//! ```
//!
//! ```lua
//! "foo"
//! ```
//!
//! Note: `untagged` is only supported for `ToLua` on enums where each variant contains a single
//! anonymous value.
//!
//! # TODO
//!
//! This is a list of things that I would like to work, but that don't.
//!
//! ```rust
//! # use rlua::Lua;
//! # use rlua_derive_unofficial::{FromLua, ToLua};
//! # #[derive(FromLua, ToLua)]
//! #[rlua(array)]
//! struct SimpleStruct {
//!     message: String,
//!     count: u64,
//! }
//! ```
//!
//! ```lua
//! { "foo", 37 }
//! ```


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
