use rusqlite::{Connection, Result};
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug)]
#[orm_autobind[(foo, bar)]]
struct Orm {
    foo: String,
    bar: i32,
    baz: Baz,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct Baz {
    bazzes: Vec<i32>,
}

fn foobar() -> Orm {
    Orm {
        foo: "foo1".into(),
        bar: 42,
        baz: Baz {
            bazzes: vec![1, 2, 3],
        },
    }
}

fn setup() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;
    conn.trace(Some(|query| println!("{}", query)));
    OrmDao::create_table(&conn)?;
    OrmDao::from(foobar()).insert(&conn)?;

    Ok(conn)
}

#[test]
fn select() -> Result<()> {
    let conn = setup()?;

    let daos = OrmDao::select_bar(&conn, &42)?;
    let res = daos.into_iter().map(OrmDao::into).collect::<Vec<Orm>>();
    assert_eq!(res, vec![foobar().into()]);

    assert_eq!(OrmDao::select_bar(&conn, &1)?, vec![]);

    Ok(())
}

#[test]
fn count() -> Result<()> {
    let conn = setup()?;

    assert_eq!(OrmDao::count_bar(&conn, &42)?, 1);

    assert_eq!(OrmDao::count_bar(&conn, &1)?, 0);

    Ok(())
}

#[test]
fn update() -> Result<()> {
    let conn = setup()?;

    let (id, _dao) = OrmDao::select_bar(&conn, &42)?.pop().unwrap().into();
    OrmDao::update_bar(&conn, &id.unwrap(), &69)?;

    assert_eq!(OrmDao::select_bar(&conn, &42)?, vec![]);

    let daos = OrmDao::select_bar(&conn, &69)?;
    assert_eq!(daos.len(), 1);

    Ok(())
}

#[test]
fn product() -> Result<()> {
    let conn = setup()?;

    let daos = OrmDao::select_foo_and_bar(&conn, &"foo1".into(), &42)?;
    let res = daos.into_iter().map(OrmDao::into).collect::<Vec<Orm>>();
    assert_eq!(res, vec![foobar().into()]);

    assert_eq!(OrmDao::select_foo_and_bar(&conn, &"foo1".into(), &1)?, vec![]);
    assert_eq!(OrmDao::select_foo_and_bar(&conn, &"foo2".into(), &42)?, vec![]);

    Ok(())
}
