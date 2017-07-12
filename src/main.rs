extern crate iron;
extern crate router;
extern crate params;

#[macro_use]
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;
use params::{Params, Value};

fn is_slackbot(name: &String) -> bool {
    name == "slackbot"
}

fn gen_response(name: &String) -> String {
    json!({"text": "Hello, ".to_string() + name}).to_string()
}

fn main() {

    fn top_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }

    fn hoge_handler(req: &mut Request) -> IronResult<Response> {

        let map = req.get_ref::<Params>().unwrap();

        match map.find(&["user_name"]) {
            Some(&Value::String(ref user_name)) if !(is_slackbot(user_name)) => {
                Ok(Response::with((status::Ok, gen_response(user_name))))
            }
            _ => Ok(Response::with((status::Ok, iron::status::NotFound))),
        }
    }

    let mut router = Router::new();
    router.get("/", top_handler, "top");
    router.post("/hoge", hoge_handler, "hoge");

    Iron::new(router).http("0.0.0.0:3000").unwrap();

}
