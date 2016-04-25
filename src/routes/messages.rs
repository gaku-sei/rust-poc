use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

use services::common::*;
use services::rest::messages::*;

pub fn all(_: &mut Request) -> IronResult<Response> {
    let messages = Message::all().unwrap();
    let json = json::encode(&messages).unwrap();
    Ok(Response::with((status::Ok, json)))
}

pub fn find(req: &mut Request) -> IronResult<Response> {
    let id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    match id.parse::<i32>() {
        Ok(id) => match Message::find(id) {
            Ok(message) => {
                let json = json::encode(&message).unwrap();
                Ok(Response::with((status::Ok, json)))
            },
            Err(_) => Ok(Response::with((status::NotFound, "not found"))),
        },
        Err(_) => Ok(Response::with((status::BadRequest, "bad request")))
    }
    // if let Some(id) = id.parse::<i32>().ok() {
    //     if let Some(message) = Message::find(id).ok() {
    //         let json = json::encode(&message).unwrap();
    //         Ok(Response::with((status::Ok, json)))
    //     } else {
    //         Ok(Response::with((status::NotFound, "message not found")))
    //     }
    // } else {
    //     Ok(Response::with((status::BadRequest, "bad request")))
    // }
}
