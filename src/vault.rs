use std::path::{Path};
use rusqlite;
use rusqlite::SqliteConnection;

use rustc_serialize::json::{Json};

use glacier;

pub struct Vault {
  name: String
}

impl Vault {
  pub fn new(name: String) -> Vault {
    Vault {
      name: name
    }
  }

  pub fn save(&self) {
    let path = Path::new("./test-sqlite.db");
    let conn = SqliteConnection::open(&path).unwrap();

    conn.execute("INSERT INTO vaults (name)
                  VALUES ($1)",
                  &[&self.name]).unwrap();
  }
}

pub fn sync_vaults_for_user(access_key: &str, secret_key: &str) {
  let mut vaults_array: Vec<Json> = glacier::list_vaults_for_user(access_key, secret_key).unwrap();

  for v in vaults_array.iter() {
    let vault = v.as_object().unwrap();
    let name  = format!("{}", vault.get("VaultName").unwrap().as_string().unwrap());
    let arn   = format!("{}", vault.get("VaultARN").unwrap().as_string().unwrap());
    let files = format!("{}", vault.get("NumberOfArchives").unwrap());
    println!("vault.size: {}", vault.get("SizeInBytes").unwrap());
    println!("vault.created_at: {}", vault.get("CreationDate").unwrap());

    Vault::new(name).save();
  }
}
