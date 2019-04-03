use rlua_derive_unofficial::ToLua;

use rlua::ToLua;

#[derive(Debug, Clone, PartialEq, ToLua)]
struct Named {
    is_rusty: bool,
}

#[derive(Debug, Clone, PartialEq, ToLua)]
struct Unnamed(u8, String);

#[derive(Debug, Clone, PartialEq, ToLua)]
struct Unit;

#[test]
fn basic_table() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Named{ is_rusty: true };

    {
        let table = table.clone();
        lua.context(|ctx| {
            table.to_lua(ctx).unwrap();
        });
    }

    assert_eq!(table, table);
}

#[test]
fn basic_unnamed() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Unnamed(1,"2".to_string());

    {
        let table = table.clone();
        lua.context(|ctx| {
            table.to_lua(ctx).unwrap();
        });
    }

    assert_eq!(table, table);
}

#[test]
fn basic_unit() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Unit;

    {
        let table = table.clone();
        lua.context(|ctx| {
            table.to_lua(ctx).unwrap();
        });
    }

    assert_eq!(table, table);
}
