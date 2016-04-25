extern crate iron;
extern crate router;
extern crate rusqlite;
extern crate rustc_serialize;

mod routes;
mod services;

use iron::prelude::*;
use router::Router;
use routes::*;

fn main() {
    let mut router = Router::new();

    router.get("/messages", messages::all);
    router.get("/messages/:id", messages::find);

    Iron::new(router).http("localhost:3000").unwrap();
}
