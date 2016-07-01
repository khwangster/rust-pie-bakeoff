extern crate iron;
use iron::prelude::*;

#[macro_use]
extern crate router;

extern crate hyper;
use hyper::client::Client;
use std::io::Read;

extern crate rustc_serialize;

extern crate persistent;
use persistent::State;
use iron::typemap::Key;

use std::collections::HashMap;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::thread;

use r2d2_redis::RedisConnectionManager;

use redis::Commands;

mod endpoints;
mod response;
mod pies;
mod cache;

fn main() {
    let router = router!(
        get "/" => endpoints::hello_world,
        get "/pies" => endpoints::pies,
        get "/pies/recommend" => endpoints::recommend,
        get "/pie/:pie_id" => endpoints::pie,
        post "/pie/:pie_id/purchases" => endpoints::purchase
    );

    let mut chain = Chain::new(router);
    let pies = boot();
    chain.link(State::<cache::AllPies>::both(pies));
//    chain.link(State::<cache::IdIndex>::both(make_id_index(pies)));

    Iron::new(chain).http("localhost:3000").unwrap();

}

fn boot() -> Vec<pies::Pie> {
    let client = Client::new();
    let mut res = client.get("http://stash.truex.com/tech/bakeoff/pies.json")
        .send().unwrap();

    let mut json = String::new();
    res.read_to_string(&mut json)
        .expect("failed to read pies.json");

    println!("{}", json);
    pies::new(json)
}

//fn connect_redis() -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
//    let config = Default::default();
//    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
//    let pool = r2d2::Pool::new(config, manager).unwrap();
//    pool
//}
//
//fn update_redis(pies: &Vec<pies::Pie>) {
//    pies;
//}
//
//fn make_id_index(pies: &Vec<pies::Pie>) -> HashMap<u64, &pies::Pie> {
//    let mut hash = HashMap::new();
//    hash.insert(pies[0].id, &pies[0]);
//    hash
//}