use std::path::{Path};
use rusqlite;
use rusqlite::SqliteConnection;

pub fn init() {
  create_users_table();
  create_vaults_table();
}

fn create_users_table() -> Result<i32, rusqlite::SqliteError> {
  let path = Path::new("./test-sqlite.db");
  let conn = SqliteConnection::open(&path).unwrap();

  let mut stmt: rusqlite::SqliteStatement = try!(conn.prepare("CREATE TABLE users (
                id              SERIAL PRIMARY KEY,
                username        VARCHAR NOT NULL,
                password        VARCHAR NOT NULL,
                salt            VARCHAR NOT NULL
                )"));
  Ok(try!(stmt.execute(&[])))
}

fn create_vaults_table() -> Result<i32, rusqlite::SqliteError> {
  let path = Path::new("./test-sqlite.db");
  let conn = SqliteConnection::open(&path).unwrap();

  let mut stmt: rusqlite::SqliteStatement = try!(conn.prepare("CREATE TABLE vaults (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL
                )"));
  Ok(try!(stmt.execute(&[])))
}
