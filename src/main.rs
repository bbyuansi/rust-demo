extern crate iron;
extern crate rustc_serialize;
extern crate rand;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rustc_serialize::json;
use rand::Rng;

#[derive(RustcEncodable)]
struct Greeting {
    msg: String,
    status: i32,
}

fn pick_response() -> String {
    let rand = rand::thread_rng().gen_range(1, 4);

    let pr = match rand {
        1 => "hello, world",
        2 => "you are so lucky",
        3 => "you failed",
        _ => "fuck this"
    };
    pr.to_string()
}

fn main() {
    println!("{:?}", pick_response());
    Iron::new(|_: &mut Request| {
        let content_type = "application/json".parse::<Mime>().unwrap();
        let greeting = Greeting { msg: "heheheda".to_string(), status: 1};
        Ok(Response::with((content_type, status::Ok, json::encode(&greeting).unwrap())))
    }).http("localhost:3000").unwrap();
}