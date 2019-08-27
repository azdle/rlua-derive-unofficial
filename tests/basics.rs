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

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
enum NumOrStr {
    Num(u64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
#[rlua(tag = "type")]
enum TaggedNumOrStr {
    Num(u64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
#[rlua(tag = "type", content = "val")]
enum TaggedContentNumOrStr {
    Num(u64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, ToLua)]
#[rlua(tag = "type", content = "val")]
enum GenericEnum<T> {
    Yes(T),
    No(T),
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

#[test]
fn basic_enum_to() {
    use rlua::Lua;
    let lua = Lua::new();

    let nos = NumOrStr::Num(37);

    lua.context(|l| {
        l.globals().set("nos", nos).unwrap();
        l.load(
            r#"
            assert(type(nos) == "table")
            assert(nos.num == 37)
        "#,
        )
        .exec()
        .unwrap()
    });
}

#[test]
fn basic_enum_from() {
    use rlua::Lua;
    let lua = Lua::new();

    let nos = lua.context(|l| {
        l.load(
            r#"
            return {num = 63}
        "#,
        )
        .eval::<NumOrStr>()
        .unwrap()
    });

    assert_eq!(nos, NumOrStr::Num(63));
}

#[test]
fn tagged_content_enum_to() {
    use rlua::Lua;
    let lua = Lua::new();

    let tnos = TaggedContentNumOrStr::Num(37);

    lua.context(|l| {
        l.globals().set("tnos", tnos).unwrap();
        l.load(
            r#"
            assert(type(tnos) == "table", "not table")
            assert(tnos.type ~= nil, "type not specified")
            assert(tnos.type == "num", "type not correct")
            assert(type(tnos) == "table", "value table not found")
            assert(tnos.val ~= nil, "value not found")
            assert(tnos.val == 37, "value in not found")
        "#,
        )
        .exec()
        .unwrap()
    });
}
#[test]
fn tagged_content_enum_from() {
    use rlua::Lua;
    let lua = Lua::new();

    let nos = lua.context(|l| {
        l.load(
            r#"
            return {type = "num", val = 63}
        "#,
        )
        .eval::<TaggedContentNumOrStr>()
        .unwrap()
    });

    assert_eq!(nos, TaggedContentNumOrStr::Num(63));
}

#[test]
fn understand_stuff() {
    use rlua::Lua;
    let lua = Lua::new();

    let () = lua.context(|l| {
        l.load(
            r#"
            return {}
        "#,
        )
        .eval::<()>()
        .unwrap()
    });

    assert_eq!((), ());
}
