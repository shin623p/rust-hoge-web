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
    json!({
              "text": n.to_string() + " は素数です。"
          })
            .to_string()
}

fn filter_prime(num: u32) -> Option<u32> {
    match num {
        0 => None,
        1 => None,
        2 => Some(2),
        _ if seive(&num).ends_with(&[num]) => Some(num),
        _ => None,
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
                go((|| {
                        p.push(x);
                        p
                    })(),
                   (|| {
                        n.retain(|&i| i % x != 0);
                        n
                    })(),
                   num)
            }
            _ => {
                (|| {
                     p.append(&mut n);
                     p
                 })()
            }
        }
    }

    go(vec![2], nums, num)
}

fn capture_prime_num(text: Option<&Value>) -> Option<u32> {
    if let Some(&Value::String(ref t)) = text {
        capture_num(t).and_then(filter_prime)
    } else {
        None
    }
}

fn capture_num(text: &String) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures(text)
        .and_then(|caps| caps[0].parse::<u32>().ok())
}

fn hoge_handler(req: &mut Request) -> IronResult<Response> {

    //let res_ok = |text| Ok(Response::with((status::Ok, text)));
    //let res_400 = || Ok(Response::with((status::Ok, iron::status::BadRequest)));

    let map = req.get_ref::<Params>().unwrap();

    if let Some(text) = match is_slackbot(map.find(&["user_name"])) {
           true => None,
           false => capture_prime_num(map.find(&["text"])).map(gen_response),
       } {
        Ok(Response::with((status::Ok, text)))
    } else {
        Ok(Response::with((status::Ok, iron::status::BadRequest)))
    }
}


fn main() {

    let top_handler = |_: &mut Request| Ok(Response::with((status::Ok, "Hello, world!")));

    // fn top_handler(_: &mut Request) -> IronResult<Response> {
    //     Ok(Response::with((status::Ok, "Hello, world!")))
    // }

    let mut router = Router::new();
    router.get("/", top_handler, "top");
    router.post("/hoge", hoge_handler, "hoge");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
