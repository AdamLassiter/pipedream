use rusqlite::{Connection, Result};
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug)]
#[orm_autobind]
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

    let mut ress = OrmDao::select_by_bar(&conn, &42)?;
    ress.iter_mut().for_each(|res| res.id = None);
    assert_eq!(ress, vec![foobar().into()]);

    assert_eq!(OrmDao::select_by_bar(&conn, &1)?, vec![]);

    Ok(())
}
