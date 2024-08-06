use rusqlite::{Connection, Result};
use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct SubDao {
    prop: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[orm_bind { name: "$.name", prop: "$.sub.prop"}]
struct Dao {
    name: String,
    sub: SubDao,
}

#[test]
fn test() -> Result<()> {
    let dao = || Dao {
        name: "name1".into(),
        sub: SubDao {
            prop: "prop1".into(),
        },
    };

    let mut conn = Connection::open_in_memory()?;
    conn.trace(Some(|query| println!("{}", query)));
    Dao::create_table(&conn)?;
    dao().insert(&conn)?;
    // assert_eq!(Dao::query(&conn, 1)?, vec![dao()]);
    assert_eq!(Dao::query_by_name(&conn, "name1")?, vec![dao()]);
    assert_eq!(Dao::query_by_prop(&conn, "prop1")?, vec![dao()]);

    Ok(())
}
