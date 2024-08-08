use rusqlite::{Connection, Result};
use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[orm_bind ({ foo: "$.foo", bar: "$.bar", baz: "$.baz[0]" }, [ (foo, bar) ])]
struct Orm {
    foo: Foo,
    bar: Bar,
    baz: Baz,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Foo(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Bar {
    bar: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Baz(String, u64);

fn foobar() -> Orm {
    Orm {
        foo: Foo("foo1".into()),
        bar: Bar { bar: "bar1".into() },
        baz: Baz("baz1".into(), 42),
    }
}

fn setup() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;
    conn.trace(Some(|query| println!("{}", query)));
    Orm::create_table(&conn)?;
    foobar().insert(&conn)?;

    Ok(conn)
}

#[test]
fn test_query_by_id() -> Result<()> {
    let conn = setup()?;

    assert_eq!(Orm::query_by_id(&conn, 1)?, Some(foobar()));

    assert_eq!(Orm::query_by_id(&conn, 2)?, None);

    Ok(())
}
#[test]
fn test_query_by_binding() -> Result<()> {
    let conn = setup()?;

    assert_eq!(Orm::query_by_foo(&conn, &"foo1")?, vec![foobar()]);
    assert_eq!(
        Orm::query_by_bar(&conn, &Bar { bar: "bar1".into() })?,
        vec![foobar()]
    );
    assert_eq!(Orm::query_by_baz(&conn, &"baz1")?, vec![foobar()]);

    assert_eq!(Orm::query_by_foo(&conn, &"foo2")?, vec![]);
    assert_eq!(
        Orm::query_by_bar(&conn, &Bar { bar: "bar2".into() })?,
        vec![]
    );
    assert_eq!(Orm::query_by_baz(&conn, &"baz2")?, vec![]);

    Ok(())
}
#[test]
fn test_query_by_product() -> Result<()> {
    let conn = setup()?;

    assert_eq!(
        Orm::query_by_foo_and_bar(&conn, &"foo1", &Bar { bar: "bar1".into() })?,
        vec![foobar()]
    );

    assert_eq!(
        Orm::query_by_foo_and_bar(&conn, &"foo2", &Bar { bar: "bar1".into() })?,
        vec![]
    );
    assert_eq!(
        Orm::query_by_foo_and_bar(&conn, &"foo1", &Bar { bar: "bar2".into() })?,
        vec![]
    );

    Ok(())
}
