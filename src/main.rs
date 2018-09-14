extern crate hyper;
extern crate futures;

use hyper::{Body, Request, Response, Server};
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use futures::future;
use hyper::{Method, StatusCode};
use futures::Stream;
use hyper::Chunk;

type Boxfut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn echo(req: Request<Body>) -> Boxfut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo")
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body()
        }
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req.into_body()
                .map(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });
            *response.body_mut() = Body::wrap_stream(mapping)
        }
        (&Method::POST, "/echo/reverse") => {
            let reversed = req.into_body()
                .concat2()
                .map(move |chunk| {
                    let body = chunk.iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<u8>>();
                    *response.body_mut() = Body::from(body);
                    response
                });
            return Box::new(reversed);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND
        }
    }

    Box::new(future::ok(response))
}

fn main() {
    let phrase: &'static [u8] = b"hello, world";
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(||service_fn(echo))
        .map_err(|e| eprintln!("server error: {:?}!", e));
    println!("listen on http://{}", addr);

    rt::run(server)
}