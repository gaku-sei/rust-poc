use rusqlite::{Error, Result};
use services::common::*;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Message {
    id: Option<i32>,
    room_id: Option<i32>,
    content: Option<String>
}

impl Message {
    pub fn new(id: Option<i32>, room_id: Option<i32>, content: Option<String>) -> Message {
        Message { id: id, room_id: room_id, content: content }
    }

    #[allow(dead_code)]
    pub fn init(room_id: Option<i32>, content: Option<String>) -> Message {
        Message { id: None, room_id: room_id, content: content }
    }
}

impl Model<Message> for Message {
    fn all() -> Result<Vec<Message>> {
        let conn = connect();
        let mut stmt = try!(conn.prepare("SELECT id, room_id, content FROM messages"));

        let messages = try!(stmt.query_map(&[], |row| {
            Message::new(Some(row.get(0)), Some(row.get(1)), Some(row.get(2)))
        }));

        Ok(messages.map(|message| message.unwrap()).collect())
    }

    fn find(id: i32) -> Result<Message> {
        let conn = connect();
        let mut stmt = try!(conn.prepare("SELECT id, room_id, content FROM messages WHERE id=$1 LIMIT 1"));

        let mut messages = try!(stmt.query_map(&[&id], |row| {
            Message::new(Some(row.get(0)), Some(row.get(1)), Some(row.get(2)))
        }));

        if let Some(message) = messages.next() {
            Ok(message.unwrap())
        } else {
            Err(Error::QueryReturnedNoRows)
        }
    }
}

impl CRUDable for Message {
    fn insert(self) -> EmptyResult {
        let conn = connect();
        let room_id = self.room_id.unwrap_or(1);
        let content = self.content.unwrap_or("default_content".to_string());

        try!(conn.execute("INSERT INTO messages (room_id, content) VALUES ($1, $2)", &[&room_id, &content]));

        Ok(())
    }
}
