use std::path::{Path};
use rusqlite;
use rusqlite::SqliteConnection;

use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};
use crypto::bcrypt::bcrypt;
use rand::{OsRng, Rng};

use std;

pub struct User {
  username: String,
  password: String,
  encrypted_password: Option<Vec<u8>>,
  salt: Option<String>
}

impl User {
  pub fn new(username: String, password: String) -> User {
    User {
      username: username,
      password: password,
      encrypted_password: None,
      salt: None
    }
  }

  fn encrypt(&self) -> User {
    let mut encrypted_password_u8 = [0u8; 24];
    let mut salt: String = User::generate_salt();
    bcrypt(5, salt.as_bytes(), self.password.as_bytes(), &mut encrypted_password_u8[..]);

    let mut encrypted_password: Vec<u8> = vec![];

    for c in encrypted_password_u8.iter() {
      encrypted_password.push(*c);
    };

    // println!("DEBUG: salt: {}", encrypted_password.to_base64(STANDARD));
    User {
      username: self.username.clone(),
      password: self.password.clone(),
      encrypted_password: Some(encrypted_password),
      salt: Some(salt)
    }
  }

  pub fn save(&self) {
    // create_users_table();

    let crypted: User = self.encrypt();

    let path = Path::new("./test-sqlite.db");
    let conn = SqliteConnection::open(&path).unwrap();

    conn.execute("INSERT INTO users (username, password, salt)
                  VALUES ($1, $2, $3)",
                 &[&crypted.username.clone(), &crypted.encrypted_password.clone().unwrap().to_base64(STANDARD), &crypted.salt.clone().unwrap()]).unwrap();
  }

  pub fn generate_salt() -> String {
    let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");
    // let mut key: Vec<u8> = vec![]; //Vec::from_elem(16, 0u8);
    // let mut key: Vec<u8> = Vec::with_capacity(16);
    let mut key = [0u8; 10];
    gen.fill_bytes(&mut key[..]);
    println!("Key: {}", key.to_base64(STANDARD));
    key.to_base64(STANDARD)
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

  pub fn authenticate_user(username: &String, password: &String) {

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

}
