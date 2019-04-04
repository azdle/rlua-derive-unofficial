use rlua_derive_unofficial::{ToLua, FromLua};

use rlua::{ToLua,FromLua};

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
struct Named {
    is_rusty: bool,
}

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
struct Unnamed(u8, String);

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
struct Unit;

#[test]
fn basic_table() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Named{ is_rusty: true };

    let round_trip_table = {
        let table = table.clone();
        lua.context(|ctx| {
            let lt = table.to_lua(ctx).unwrap();
            Named::from_lua(lt, ctx).unwrap()
        })
    };

    assert_eq!(table, round_trip_table);
}

#[test]
fn basic_unnamed() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Unnamed(1,"2".to_string());

    let round_trip_table = {
        let table = table.clone();
        lua.context(|ctx| {
            let lt = table.to_lua(ctx).unwrap();
            Unnamed::from_lua(lt, ctx).unwrap()
        })
    };

    assert_eq!(table, round_trip_table);
}

#[test]
fn basic_unit() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Unit;

    let round_trip_table = {
        let table = table.clone();
        lua.context(|ctx| {
            let lt = table.to_lua(ctx).unwrap();
            Unit::from_lua(lt, ctx).unwrap()
        })
    };

    assert_eq!(table, round_trip_table);
}
