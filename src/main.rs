extern crate iron;
extern crate router;
extern crate params;
extern crate regex;

#[macro_use]
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;
use params::{Params, Value};
use regex::Regex;

fn is_slackbot(name: Option<&Value>) -> bool {
    match name {
        Some(&Value::String(ref n)) => n == "slackbot",
        _ => false,
    }
}

fn gen_response(n: u32) -> String {
    //json!({"text": "Hello, ".to_string() + name}).to_string()
    json!({
              "text": n.to_string() + " は素数です。"
          })
            .to_string()
}

fn is_prime(num: u32) -> bool {
    match num {
        1 => false,
        2 => true,
        _ => seive(&num).last() == Some(&num),
    }
}

fn seive(num: &u32) -> Vec<u32> {
    let nums: Vec<u32> = (1..)
        .map(|i| i * 2 + 1)
        .take(((num - 1) / 2) as usize)
        .collect::<Vec<_>>();

    fn go(mut p: Vec<u32>, mut n: Vec<u32>, num: &u32) -> Vec<u32> {
        match n.first() {
            Some(&x) if x * x <= *num => {
                p.push(x);
                n.retain(|&i| i % x != 0);
                go(p, n, num)
            }
            _ => {
                p.append(&mut n);
                p
            }
        }
    }

    go(vec![2], nums, num)
}

fn capture_prime_num(text: Option<&Value>) -> Option<u32> {
    if let Some(&Value::String(ref t)) = text {
        match capture_num(t) {
            Some(n) if is_prime(n) => Some(n),
            _ => None,
        }
    } else {
        None
    }
}

fn capture_num(text: &String) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures(text)
        .and_then(|caps| caps[0].parse::<u32>().ok())
}

fn main() {

    fn top_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }

    fn hoge_handler(req: &mut Request) -> IronResult<Response> {

        let map = req.get_ref::<Params>().unwrap();

        if is_slackbot(map.find(&["user_name"])) {
            return Ok(Response::with((status::Ok, iron::status::NotFound)));
        }

        match capture_prime_num(map.find(&["text"])) {
            Some(n) => {
                println!("Prime number: {:?}", n);
                Ok(Response::with((status::Ok, gen_response(n))))
            }
            _ => Ok(Response::with((status::Ok, iron::status::NotFound))),
        }
    }

    let mut router = Router::new();
    router.get("/", top_handler, "top");
    router.post("/hoge", hoge_handler, "hoge");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
