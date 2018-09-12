extern crate iron;
extern crate rustc_serialize;
extern crate rand;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rustc_serialize::json;
use rand::Rng;
use router::Router;

#[derive(RustcEncodable)]
struct Greeting {
    msg: String,
    status: i32,
}

fn get_greeting(req: &mut Request) -> IronResult<Response> {
    let (lang, status) = pick_response("bbyyss".to_string());
    let greeting = Greeting { msg: lang, status: status };
    let out = json::encode(&greeting).unwrap();

    println!("{:?}", req);
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn pick_response(name: String) -> (String, i32) {
    let num = rand::thread_rng().gen_range(1, 4);

    let lang = match num {
        1 => format!("hello {}!", name),
        2 => format!("how are you {}?", name),
        3 => format!("how old are you, {}?", name),
        _ => format!("you are not {}, get out!", name)
    };

    (lang.to_string(), num)
}

fn main() {
    // router
    let mut router = Router::new();
    router.get("/greeting", get_greeting, "index");
    Iron::new(router).http("localhost:3000").unwrap();
    // basic
    // Iron::new(|_: &mut Request| {
    // println!("{:?}", pick_response());
    // let content_type = "application/json".parse::<Mime>().unwrap();
    // let greeting = Greeting { msg: "heheheda".to_string(), status: 1};
    // Ok(Response::with((content_type, status::Ok, json::encode(&greeting).unwrap())))
    // }).http("localhost:3000").unwrap();
}