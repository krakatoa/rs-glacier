//hyper

extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

use hyper::{Get, Post};
use hyper::uri::RequestUri::AbsolutePath;

fn hello(req: Request, mut res: Response) {
  let settings = include_bytes!("../static/settings.png");
  let index_html = include_bytes!("../static/index.html");

  match req.uri {
    AbsolutePath(ref path) => match (&req.method, &path[..]) {
      (&Get, "/index.html") => {
        res.send(index_html);
        return;
      },
      (&Get, "/settings.png") => {
        res.send(settings);
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
  Server::http(hello).listen("127.0.0.1:3000").unwrap();
}
