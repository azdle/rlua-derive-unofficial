use rlua_derive_unofficial::ToLua;

use rlua::ToLua;

//#[derive(ToLua, FromLua)]
#[derive(Debug, Clone, PartialEq, ToLua)]
struct Table {
    is_rusty: bool,
}

//#[derive(ToLua, FromLua)]
#[derive(Debug, Clone, PartialEq, ToLua)]
struct List {
    first_thing: u8,
    second_thing: String,
}

#[test]
fn basic_table() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = Table{ is_rusty: true };

    {
        let table = table.clone();
        lua.context(|ctx| {
            table.to_lua(ctx).unwrap();
        });
    }

    assert_eq!(table, table);
}

#[test]
fn basic_list() {
    use rlua::Lua;
    let lua = Lua::new();

    let table = List{
        first_thing: 1,
        second_thing: "2".to_string(),
    };

    {
        let table = table.clone();
        lua.context(|ctx| {
            table.to_lua(ctx).unwrap();
        });
    }

    assert_eq!(table, table);
}
