extern crate iron;
use iron::prelude::*;
use iron::Protocol;

#[macro_use]
extern crate router;

extern crate hyper;
use hyper::client::Client;
use std::io::Read as io_read;

extern crate rustc_serialize;

extern crate persistent;
use persistent::{Read};

use std::cmp::Ordering;

use std::collections::HashMap;
use std::collections::HashSet;

extern crate bit_vec;
use bit_vec::BitVec;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;

use r2d2_redis::RedisConnectionManager;

extern crate url;

extern crate mustache;

extern crate num_cpus;

mod endpoints;
mod response;
mod pies;
mod cache;
mod pie_state;

fn main() {
    let router = router!(
        get "/" => endpoints::hello_world,
        get "/hello_world" => endpoints::hello_world,
        get "/pies" => endpoints::pies,
        get "/pies/recommend" => endpoints::recommend,
        get "/pie/:pie_id" => endpoints::pie,
        get "/pies/:pie_id" => endpoints::pie,
        any "/pie/:pie_id/purchases" => endpoints::purchase,
        post "/pies/:pie_id/purchases" => endpoints::purchase
    );

    let mut chain = Chain::new(router);
    let pies = parse_pie_json();
    let sorted_pies = make_price_ordered(&pies);
    chain.link_before(Read::<cache::LabelBitVec>::one(make_label_bitvec(&sorted_pies)));
    chain.link_before(Read::<cache::IdIndex>::one(make_id_index(&sorted_pies)));
    chain.link_before(Read::<cache::SortedPies>::one(sorted_pies));

    let redis = connect_redis();
    chain.link_before(Read::<cache::Redis>::one(redis.clone()));
    update_redis(&pies, &redis);

    Iron::new(chain).listen_with("0.0.0.0:31415",
                                 100 * ::num_cpus::get(),
                                 Protocol::Http,
                                 None).unwrap();
}

fn parse_pie_json() -> Vec<pies::Pie> {
    let client = Client::new();
    let mut res = client.get("http://stash.truex.com/tech/bakeoff/pies.json")
        .send().unwrap();

    let mut json = String::new();
    res.read_to_string(&mut json)
        .expect("failed to read pies.json");

    println!("{}\n", json);
    pies::new(json)
}

fn connect_redis() -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
//    let config = Default::default();
    let config = r2d2::Config::builder()
        .pool_size(100 * ::num_cpus::get() as u32)
        .build();
    let manager = RedisConnectionManager::new("redis://localhost:6379").unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();
    pool
}

fn update_redis(pies: &Vec<pies::Pie>, pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>) {
    for pie in pies {
        pie_state::set_remaining(pool, pie)
    }
}

fn make_id_index(pies: &Vec<pies::Pie>) -> HashMap<u64, (pies::Pie, usize)> {
    let mut hash = HashMap::new();
    for (i, pie) in pies.iter().enumerate() {
        hash.insert(pie.id, (pie.clone(), i));
    }
    hash
}

fn make_label_bitvec(pies: &Vec<pies::Pie>) -> HashMap<String, BitVec> {
    let mut label_set = HashSet::new();
    let mut hash = HashMap::new();

    for pie in pies {
        for label in &pie.labels {
            label_set.insert(label);
        }
    }

    for label in label_set {
        let mut bv = BitVec::from_elem(pies.len(), false);
        for (i, pie) in pies.iter().enumerate() {
            if pie.labels.contains(label) {
                bv.set(i, true);
            }
        }
        hash.insert(label.clone(), bv);
    }

    println!("bitvecs {:?}\n", hash);
    hash
}

fn make_price_ordered(pies: &Vec<pies::Pie>) -> Vec<pies::Pie> {
    let mut vec = pies.clone();
    vec.sort_by( |a, b|
        b.price_per_slice.partial_cmp(&a.price_per_slice)
            .unwrap_or(Ordering::Equal)
    );
    println!("ordered pies {:?}\n", vec);
    vec
}