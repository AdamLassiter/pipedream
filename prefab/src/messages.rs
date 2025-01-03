use pipedream_domain::message::{MessageDao, MessageLog};

use crate::Prefabricated;

impl Prefabricated for MessageLog {
    fn initialise(conn: &rusqlite::Connection) {
        MessageDao::drop_table(conn).expect("Failed to drop table for MessageLog");
        MessageDao::create_table(conn).expect("Failed to create table for MessageLog");
    }
}
