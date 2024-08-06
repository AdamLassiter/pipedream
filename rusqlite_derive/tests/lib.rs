use rusqlite::{Connection, Result};
use rusqlite_derive::json_sql;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SubDao {
    prop: String,
}

#[derive(Serialize, Deserialize)]
#[json_sql { name: "$.name", prop: "$.sub.prop"}]
struct Dao {
    name: String,
    sub: SubDao,
}

#[test]
fn test() -> Result<()> {
    let dao = Dao {
        name: "name".into(),
        sub: SubDao {
            prop: "prop".into(),
        },
    };

    let mut conn = Connection::open_in_memory()?;
    conn.trace(Some(|query| println!("{}", query)));
    Dao::create_table(&conn)?;
    dao.insert(&conn)?;
    Dao::query(&conn, 1)?;
    Dao::query_by_name(&conn, "name")?;

    Ok(())
}
