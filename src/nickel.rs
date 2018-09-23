extern crate chrono;
#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter};
use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;
use std::collections::HashMap;


fn hello_world() -> String {
    "hello, world".to_string()
}

fn show_time() {
    let time: DateTime<Local> = Local::now();
    println!("now is {}", time);
}

fn log_message(filename: &'static str, message: &'static [u8]) -> io::Result<()> {
    let mut f = try!(File::create(filename));
    try!(f.write_all(message));
    Ok(())
}

fn nickel() {
    let mut server = Nickel::new();

//    match log_message("aa.log", b"something logs") {
//        Ok(_) => println!("aa.log created!"),
//        Err(_) => println!("file created failed"),
//    }

    show_time();

    server.get("/", middleware! {|_, response|
        let mut data = HashMap::new();
        data.insert("name", "biyuansi");
        return response.render("resource/templates/hello.html", &data);
    });

    server.listen("127.0.0.1:3000");
}