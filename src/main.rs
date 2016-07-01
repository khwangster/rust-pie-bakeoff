extern crate iron;
use iron::prelude::*;

#[macro_use]
extern crate router;

extern crate hyper;
use hyper::client::Client;
use std::io::Read as io_read;

extern crate rustc_serialize;

extern crate persistent;
use persistent::{State, Read, Write};
use iron::typemap::Key;

use std::collections::HashMap;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::thread;

use r2d2_redis::RedisConnectionManager;
use redis::Commands;

extern crate url;
use url::{Url, Host};

mod endpoints;
mod response;
mod pies;
mod cache;
mod pie_state;

fn main() {
    let router = router!(
        get "/" => endpoints::hello_world,
        get "/pies" => endpoints::pies,
        get "/pies/recommend" => endpoints::recommend,
        get "/pie/:pie_id" => endpoints::pie,
        get "/pies/:pie_id" => endpoints::pie,
        any "/pie/:pie_id/purchases" => endpoints::purchase,
        post "/pies/:pie_id/purchases" => endpoints::purchase
    );

    let mut chain = Chain::new(router);
    let pies = parse_pie_json();
    chain.link_before(Read::<cache::IdIndex>::one(make_id_index(&pies)));
    chain.link_before(Read::<cache::AllPieId>::one(make_id_vec(&pies)));
    chain.link_before(Read::<cache::LabelIndex>::one(make_label_index(&pies)));

    let redis = connect_redis();
    chain.link_before(Read::<cache::Redis>::one(redis.clone()));
    update_redis(&pies, &redis);

//    pie_state::purchase_pie(&redis, &pies[0], "ken".to_string(), 1);
//    pie_state::pie_purchases(&redis, &pies[0]);

    Iron::new(chain).http("localhost:3000").unwrap();

}

fn parse_pie_json() -> Vec<pies::Pie> {
    let client = Client::new();
    let mut res = client.get("http://stash.truex.com/tech/bakeoff/pies.json")
        .send().unwrap();

    let mut json = String::new();
    res.read_to_string(&mut json)
        .expect("failed to read pies.json");

    println!("{}", json);
    pies::new(json)
}

fn connect_redis() -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();
    pool
}

fn update_redis(pies: &Vec<pies::Pie>, pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>) {
    for pie in pies {
        pie_state::set_remaining(pool, pie)
    }
}

fn make_id_index(pies: &Vec<pies::Pie>) -> HashMap<u64, pies::Pie> {
    let mut hash = HashMap::new();
    for pie in pies {
        hash.insert(pie.id, pie.clone());
    }
    hash
}

fn make_label_index(pies: &Vec<pies::Pie>) -> HashMap<String, Vec<pies::Pie>> {
    let mut hash = HashMap::new();
    for pie in pies {
        for label in &pie.labels {
            let vec = hash.entry(label.clone()).or_insert(vec![]);
            vec.push(pie.clone());
        }
    }
    hash
}

fn make_id_vec(pies: &Vec<pies::Pie>) -> Vec<u64> {
    let mut vec = Vec::new();
    for pie in pies {
        vec.push(pie.id)
    }
    vec
}