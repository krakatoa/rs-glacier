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

extern crate crypto;
extern crate rand;

extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::{Json, Parser};

//aws

extern crate aws;

mod user;
mod glacier;

use user::User;

fn hello(mut req: Request, mut res: Response) {

  let mut s = String::new();
  req.read_to_string(&mut s);

  let mut params: HashMap<String, String> = HashMap::new();
  let mut query: Vec<(String, String)> = url::form_urlencoded::parse(s.as_bytes());

  match req.uri {
    AbsolutePath(ref path) => match (&req.method, &path[..]) {
      (&Get, "/index.html") => {
        glacier::sync_vaults_for_user("AKIXXX", "XXX");
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

        User::new(username, password).save();

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
