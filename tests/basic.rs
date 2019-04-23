use rlua_derive_unofficial::{FromLua, ToLua};

use rlua::Lua;

#[derive(Clone, Debug, PartialEq, FromLua, ToLua)]
struct NamedTable {
    foo: bool,
}

#[test]
fn basic_named_table_test() {
    let lua = Lua::new();

    lua.context(|l| {
        let val = NamedTable { foo: true };
        l.globals().set("val", val.clone()).unwrap();

        let extracted_val = l.load("val.foo").eval::<bool>().unwrap();
        assert_eq!(extracted_val, true);

        let round_trip_val = l.load("val").eval::<NamedTable>().unwrap();
        assert_eq!(round_trip_val, val);
    });
}

#[rlua(as_lua_array)]
#[derive(Clone, Debug, PartialEq, FromLua, ToLua)]
struct NamedArray {
    #[rlua(index = 2)]
    foo: bool,
}

#[test]
fn basic_named_array_test() {
    let lua = Lua::new();

    lua.context(|l| {
        let val = NamedArray { foo: true };
        l.globals().set("val", val.clone()).unwrap();

        let extracted_val = l.load("val[2]").eval::<bool>().unwrap();
        assert_eq!(extracted_val, val.foo);

        let round_trip_val = l.load("val").eval::<NamedArray>().unwrap();
        assert_eq!(round_trip_val, val);
    });
}

#[derive(Clone, Debug, PartialEq, FromLua, ToLua)]
struct UnnamedArray(u8, u8, u8);

#[test]
fn basic_unnamed_array_test() {
    let lua = Lua::new();

    lua.context(|l| {
        let val = UnnamedArray(1,2,3);
        l.globals().set("val", val.clone()).unwrap();

        let extracted_val = l.load("val[2]").eval::<u8>().unwrap();
        assert_eq!(extracted_val, val.1);

        let round_trip_val = l.load("val").eval::<UnnamedArray>().unwrap();
        assert_eq!(round_trip_val, val);
    });
}

#[derive(Clone, Debug, PartialEq, FromLua, ToLua)]
struct UnnamedTable(
    #[rlua(key = "foo")]
    u8
);

#[test]
fn basic_unnamed_table_test() {
    let lua = Lua::new();

    lua.context(|l| {
        let val = UnnamedTable(37);
        l.globals().set("val", val.clone()).unwrap();

        let extracted_val = l.load("val.foo").eval::<u8>().unwrap();
        assert_eq!(extracted_val, 37);

        let round_trip_val = l.load("val").eval::<UnnamedTable>().unwrap();
        assert_eq!(round_trip_val, val);
    });
}
