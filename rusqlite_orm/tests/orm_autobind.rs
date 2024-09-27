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

#[derive(Debug, PartialEq, Eq)]
#[orm_autobind]
struct Qux {
    orm_id: OrmId,
    qux: String,
}

#[derive(Debug, PartialEq, Eq)]
#[orm_autobind]
struct TrickyTypes {
    generic: Option<Option<i32>>,
    path: std::collections::BTreeSet<std::collections::BTreeSet<i32>>,
    array: [[i32; 1]; 1],
    byte_array: [u8; 1],
    box_str: Box<str>,
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
fn qux(orm_id: OrmId) -> Qux {
    Qux {
        orm_id,
        qux: "qux1".into(),
    }
}

fn setup() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;
    conn.trace(Some(|query| println!("{}", query)));

    OrmDao::create_table(&conn)?;
    let orm_id = OrmDao::from(foobar()).insert(&conn)?;

    QuxDao::create_table(&conn)?;
    QuxDao::from(qux(orm_id)).insert(&conn)?;

    Ok(conn)
}

#[test]
fn select() -> Result<()> {
    let conn = setup()?;

    let daos = OrmDao::select_bar(&conn, &42)?;
    let res = daos.into_iter().map(OrmDao::into).collect::<Vec<Orm>>();
    assert_eq!(res, vec![foobar().into()]);

    assert_eq!(OrmDao::select_bar(&conn, &1)?, vec![]);

    let daos = QuxDao::select_orm_id(&conn, &OrmId(1))?;
    let res = daos.into_iter().map(QuxDao::into).collect::<Vec<Qux>>();
    assert_eq!(res, vec![qux(OrmId(1)).into()]);

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
fn update_sql() -> Result<()> {
    let conn = setup()?;

    let (id, _dao) = OrmDao::select_bar(&conn, &42)?.pop().unwrap().into();

    conn.prepare(&OrmDao::update_sql(&["bar"], &["id"]))?
        .execute(rusqlite::named_params! {":id": id.unwrap().0, ":bar": 69})?;

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

    assert_eq!(
        OrmDao::select_foo_and_bar(&conn, &"foo1".into(), &1)?,
        vec![]
    );
    assert_eq!(
        OrmDao::select_foo_and_bar(&conn, &"foo2".into(), &42)?,
        vec![]
    );

    Ok(())
}

#[test]
fn left_join() -> Result<()> {
    let conn = setup()?;

    let left_join_orms = conn
        .prepare("select orm.* from orms as orm left join quxs as qux on orm.id = qux.orm_id where qux.qux = :qux")?
        .query_and_then(&[(":qux", "qux1")], serde_rusqlite::from_row::<OrmDao>)?
        .map(|res| res.unwrap().into())
        .collect::<Vec<Orm>>();
    assert_eq!(left_join_orms, vec![foobar()]);

    Ok(())
}
