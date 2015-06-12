//#[macro_use]
//extern crate log;
// extern crate env_logger;
use aws::request::ApiClient;
use aws::credentials::Credentials;
// use std::io::Read;
use rustc_serialize::json::{Json, Parser};

use rusqlite;
use rusqlite::SqliteConnection;
use std::path::{Path};

use std::io::Read;
fn create_vaults_table() -> Result<i32, rusqlite::SqliteError> {
  let path = Path::new("./test-sqlite.db");
  let conn = SqliteConnection::open(&path).unwrap();

  let mut stmt: rusqlite::SqliteStatement = try!(conn.prepare("CREATE TABLE vaults (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL
                )"));
  Ok(try!(stmt.execute(&[])))
}

fn create_vault(name: &str) {
  let path = Path::new("./test-sqlite.db");
  let conn = SqliteConnection::open(&path).unwrap();

  conn.execute("INSERT INTO vaults (name)
                VALUES ($1)",
               &[&name]).unwrap();
}

pub fn sync_vaults_for_user(access_key: &str, secret_key: &str) {
  create_vaults_table();

  // env_logger::init().unwrap();
  let cred = Credentials::create("AKIXXX", "XXX");
  let region = "us-east-1";
  let service = "glacier";

  let client = ApiClient::new(cred, region, service);
  let res = client.get("vaults");
  let mut output: String = String::new();
  res.unwrap().read_to_string(&mut output);

  let data = Json::from_str(&output).unwrap();
  // println!("data: {}", data);

  let obj = data.as_object().unwrap();
  println!("vault.list: {:?}", obj.get("VaultList").unwrap().as_array().unwrap());
  for v in obj.get("VaultList").unwrap().as_array().unwrap().iter() {
    let vault = v.as_object().unwrap();
    println!("vault: {}", vault.get("VaultName").unwrap());
    println!("vault.arn: {}", vault.get("VaultARN").unwrap());
    println!("vault.files: {}", vault.get("NumberOfArchives").unwrap());
    println!("vault.size: {}", vault.get("SizeInBytes").unwrap());
    println!("vault.created_at: {}", vault.get("CreationDate").unwrap());

    create_vault(vault.get("VaultName").unwrap().as_string().unwrap());
  }

  // println!("{:?}", output)
}

