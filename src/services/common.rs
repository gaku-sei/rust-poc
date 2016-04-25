use rusqlite::{Connection, Result};
use std::path::Path;

static CONNECTION_ERROR: &'static str = "Une erreur est survenue lors de la connection";

pub type EmptyResult = Result<()>;

pub fn connect() -> Connection {
    Connection::open(&Path::new("./db.sqlite")).ok().expect(CONNECTION_ERROR)
}

pub trait Model<T> {
    fn all() -> Result<Vec<T>>;
    fn find(i32) -> Result<T>;
}

pub trait CRUDable {
    fn insert(self) -> EmptyResult;
}
