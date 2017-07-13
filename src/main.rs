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

fn is_prime(num: &u32) -> bool {
    match *num {
        1 => false,
        2 => true,
        _ => seive(num).contains(num),
    }
}

fn seive(num: &u32) -> Vec<u32> {
    let nums: Vec<u32> = (2..num + 1).collect::<Vec<_>>();
    let primes: Vec<u32> = Vec::new();

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

    return go(primes, nums, num);
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

    fn print_iter<'a, I: Iterator<Item = &'a u32>>(iter: I) {
        // ここでは引数の"型"は明示せず、traitだけでいい
        for v in iter {
            println!("The iterator's value is {}", v);
        }
    }


    println!("{:?}", seive(&100));
    println!("{}", is_prime(&3));
    println!("{}", is_prime(&1000000));
    println!("{}", is_prime(&256));
    println!("{}", is_prime(&25));


    let hoge_iter: Vec<u32> = vec![1, 2, 3];
    print_iter(hoge_iter.iter());

    let mut router = Router::new();
    router.get("/", top_handler, "top");
    router.post("/hoge", hoge_handler, "hoge");

    //Iron::new(router).http("0.0.0.0:3000").unwrap();

}
