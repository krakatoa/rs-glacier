extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

use hyper::{Get, Post};
use hyper::uri::RequestUri::AbsolutePath;

extern crate url;
use url::form_urlencoded;
use std::collections::HashMap;
use std::io::{self, Read};

extern crate rusqlite;
use rusqlite::SqliteConnection;
use std::path::{Path};

extern crate crypto;
use crypto::bcrypt::bcrypt;

extern crate rand;
use rand::{OsRng, Rng};

extern crate rustc_serialize;
use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

fn create_user(username: &String, password: &String) {
  create_table();

  let path = Path::new("./test-sqlite.db");
  let conn = SqliteConnection::open(&path).unwrap();

  let mut encrypted_password_u8 = [0u8; 24];
  let mut salt: String = generate_salt();
  bcrypt(5, salt.as_bytes(), password.as_bytes(), &mut encrypted_password_u8[..]);

  let mut encrypted_password: Vec<u8> = vec![];

  for c in encrypted_password_u8.iter() {
    encrypted_password.push(*c);
  };

  // println!("DEBUG: salt: {}", encrypted_password.to_base64(STANDARD));

  conn.execute("INSERT INTO users (username, password, salt)
                VALUES ($1, $2, $3)",
               &[username, &encrypted_password.to_base64(STANDARD), &salt]).unwrap();
}

fn create_table() -> Result<i32, rusqlite::SqliteError> {
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

fn generate_salt() -> String {
  let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");
  // let mut key: Vec<u8> = vec![]; //Vec::from_elem(16, 0u8);
  // let mut key: Vec<u8> = Vec::with_capacity(16);
  let mut key = [0u8; 10];
  gen.fill_bytes(&mut key[..]);
  println!("Key: {}", key.to_base64(STANDARD));
  key.to_base64(STANDARD)
}

fn authenticate_user(username: &String, password: &String) {

  let path = Path::new("./test-sqlite.db");
  let conn = SqliteConnection::open(&path).unwrap();

  let mut query: String = format!("SELECT password, salt FROM users WHERE username = '{}'", username);
  // println!("DEBUG: {}", query);

  conn.query_row(&query[..], &[], |row| {
    let challenged_password: Vec<u8> = row.get(0);

    let salt_u8: Vec<u8> = row.get(1);
    let mut salt: String = String::from_utf8(salt_u8).unwrap();
    // println!("DEBUG: salt: {}", std::str::from_utf8(&salt[..]).unwrap());
    println!("DEBUG: salt: {}", salt);

    let mut gen_enc_password_u8 = [0u8; 24];
    let mut gen_enc_password: Vec<u8> = vec![];

    // bcrypt(5, &salt[..], &std::str::from_utf8(password.as_bytes()).unwrap().from_base64().unwrap()[..], &mut gen_enc_password_u8[..]);
    // bcrypt(5, &salt.as_bytes(), &password.from_base64().unwrap()[..], &mut gen_enc_password_u8[..]);
    bcrypt(5, &salt.as_bytes(), &password.as_bytes(), &mut gen_enc_password_u8[..]);

    for c in gen_enc_password_u8.iter() {
      gen_enc_password.push(*c);
    };
    // println!("gen_enc_password: {:?}", gen_enc_password.to_base64(STANDARD));
    // println!("challenged_password: {:?}", std::str::from_utf8(&challenged_password[..]).unwrap()); //.to_base64(STANDARD));

    assert_eq!(std::str::from_utf8(&challenged_password[..]).unwrap(), gen_enc_password.to_base64(STANDARD));
  }).unwrap()
}

fn hello(mut req: Request, mut res: Response) {

  let mut s = String::new();
  req.read_to_string(&mut s);

  let mut params: HashMap<String, String> = HashMap::new();
  let mut query: Vec<(String, String)> = url::form_urlencoded::parse(s.as_bytes());

  match req.uri {
    AbsolutePath(ref path) => match (&req.method, &path[..]) {
      (&Get, "/index.html") => {
        // generate_salt();
        // authenticate_user(&"chimuelo".to_string(), &"sarasa".to_string());
        let static_index_html = include_bytes!("../static/index.html");
        res.send(static_index_html);
        return;
      },
      (&Get, "/settings.png") => {
        let static_settings_png = include_bytes!("../static/settings.png");
        res.send(static_settings_png);
        return;
      },
      (&Get, "/app.js") => {
        let static_app_js = include_bytes!("../static/app.js");
        res.send(static_app_js);
        return;
      },
      (&Post, "/register") => {
        let (k, v): (String, String);
        for p in query.iter() {
          let (ref k, ref v) = *p;
          params.insert(k.clone(), v.clone());
        }

        let username: String;
        let password: String;
        // let aws_access_key_id: String;
        // let aws_secret_access_key: String;

        match params.get("user_username") {
          Some(value) => username = value.clone(),
          None => {
            println!("username not found");
            return;
          }
        }

        match params.get("user_password") {
          Some(value) => password = value.clone(),
          None => {
            println!("password not found");
            return;
          }
        }

        //match params.get("user_aws_access_key_id") {
        //  Some(value) => aws_access_key_id = value.clone(),
        //  None => {
        //    println!("aws_access_key_id not found");
        //    return;
        //  }
        //}

        //match params.get("user_aws_secret_access_key") {
        //  Some(value) => aws_secret_access_key = value.clone(),
        //  None => {
        //    println!("aws_secret_access_key not found");
        //    return;
        //  }
        //}

        create_user(&username, &password);
        println!("user: {}, pass: {}", username, password);

        return;
      },
      _ => {
        *res.status_mut() = hyper::NotFound;
        return;
      }
    },
    _ => {
      return;
    }
  };

  let mut res = res.start().unwrap();
}

fn main() {
  Server::http(hello).listen("0.0.0.0:3000").unwrap();
}
