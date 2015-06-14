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

pub fn list_vaults_for_user(access_key: &str, secret_key: &str) -> Option<Vec<Json>> {
  let cred = Credentials::create(access_key, secret_key);
  let region = "us-east-1";
  let service = "glacier";

  let client = ApiClient::new(cred, region, service);
  let res = client.get("vaults");
  let mut output: String = String::new();
  res.unwrap().read_to_string(&mut output);

  let data = Json::from_str(&output).unwrap();

  let obj = data.as_object().unwrap();
  println!("vault.list: {:?}", obj.get("VaultList").unwrap().as_array().unwrap());
  Some(obj.get("VaultList").unwrap().as_array().unwrap().clone())
}
