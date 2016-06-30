extern crate iron;
use iron::prelude::*;

#[macro_use]
extern crate router;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate hyper;
use hyper::client::Client;
use std::io::Read;

mod endpoints;
mod response;

fn main() {
    let router = router!(
        get "/" => endpoints::hello_world,
        get "/pies" => endpoints::pies,
        get "/pies/recommend" => endpoints::hello_world,
        get "/pie/:pie_id" => endpoints::pie,
        post "/pie/:pie_id/purchases" => endpoints::hello_world
    );

    boot();
//    Iron::new(router).http("localhost:3000").unwrap();
}


fn boot() {
    let client = Client::new();
    let mut res = client.get("http://stash.truex.com/tech/bakeoff/pies.json")
        .send().unwrap();

    let mut json = String::new();
    res.read_to_string(&mut json)
        .expect("failed to read pies.json");

    println!("{}", json)
}