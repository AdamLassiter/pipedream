use chrono::{DateTime, Local};
use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize, Serializer};

#[orm_autobind]
pub struct Message {
    pub timestamp: Timestamp,
    pub message: String,
}
impl Message {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            timestamp: Timestamp::unset(),
            message: message.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Timestamp {
    #[serde(serialize_with = "default_timestamp")]
    datetime: Option<DateTime<Local>>,
}
impl Timestamp {
    fn unset() -> Self {
        Self { datetime: None }
    }
}
fn default_timestamp<S>(x: &Option<DateTime<Local>>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.serialize_some(&x.or(Some(Local::now())))
}

pub struct MessageLog(pub Vec<Message>);

impl MessageLog {
    pub fn get_message_log(conn: &Connection) -> Self {
        let mut messages = MessageDao::all(conn)
            .expect("Failed to query message log")
            .into_iter()
            .map(|dao| dao.into())
            .collect::<Vec<Message>>();
        messages.sort_by_key(|m| m.timestamp.datetime);
        Self(messages)
    }

    // pub fn insert_message(conn: &Connection, message: Message) {
    //     MessageDao::from(message)
    //         .insert(conn)
    //         .expect("Failed to write message to log");
    // }
}
