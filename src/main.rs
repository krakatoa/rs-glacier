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

use std::io::{self, Read};

fn hello(mut req: Request, mut res: Response) {

  let mut s = String::new();
  req.read_to_string(&mut s);
  let params: Vec<(String, String)> = url::form_urlencoded::parse(s.as_bytes());

  match req.uri {
    AbsolutePath(ref path) => match (&req.method, &path[..]) {
      (&Get, "/index.html") => {
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
      (&Post, "/users") => {
        let (k, v): (String, String);
        for p in params.iter() {
          let (ref k, ref v) = *p;
          println!("{}: {}", k, v);
        }
        return;
      },
      /*(&Post, "/index.html") => {
        *res.status_mut() = hyper::NotFound;
        println!("should fail");
        return;
      },
      (&Post, "/echo") => (), // fall through, fighting mutable borrows
      */
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
  // res.end().unwrap();
}

fn main() {
  Server::http(hello).listen("0.0.0.0:3000").unwrap();
}
