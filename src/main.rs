//hyper

extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

//lmdb

extern crate lmdb_rs as lmdb;

use std::path::Path;
use lmdb::{EnvBuilder, DbFlags};

fn hello(_: Request, res: Response<Fresh>) {
  let mut res = res.start().unwrap();
  // res.write_all(b"Hello World!").unwrap();
  let name = read();
  // println!("It's {:?} Smith", name);
  res.write_all(name.as_bytes());
  res.end().unwrap();
}

fn read() -> String {
  let path = Path::new("test-lmdb");
  let mut env = EnvBuilder::new().open(&path, 0o777).unwrap();

  let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
  // let txn = env.new_transaction().unwrap();
  // {
  //     let db = txn.bind(&db_handle); // get a database bound to this transaction

  //     let pairs = vec![("Albert", "Einstein",),
  //                      ("Joe", "Smith",),
  //                      ("Jack", "Daniels")];

  //     for &(name, surname) in pairs.iter() {
  //         db.set(&surname, &name).unwrap();
  //     }
  // }

  // // Note: `commit` is choosen to be explicit as
  // // in case of failure it is responsibility of
  // // the client to handle the error
  // match txn.commit() {
  //     Err(_) => panic!("failed to commit!"),
  //     Ok(_) => ()
  // }

  let reader = env.get_reader().unwrap();
  let db = reader.bind(&db_handle);
  let name = db.get::<&str>(&"Smith");
  // println!("It's {} Smith", name);
  String::from(name.unwrap())
}

fn main() {
  Server::http(hello).listen("127.0.0.1:3000").unwrap();
}
