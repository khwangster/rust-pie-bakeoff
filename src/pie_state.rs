
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::thread;

use r2d2_redis::RedisConnectionManager;
use redis::Commands;

use pies;

macro_rules! remaining_key {
    ( $x:expr ) => ( format!("pie-{}-remaining", $x) )
}

pub fn set_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) {
    let conn = pool.get().expect("redis connection failed");
    let _ : () = conn.set(remaining_key!(pie.id), pie.slices).unwrap();

    let n : u64 = conn.get(remaining_key!(pie.id)).unwrap();
    println!("setting remaining for pie {} to {}", pie.name, n);
}

pub fn get_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) -> u64 {
    let conn = pool.get().expect("redis connection failed");
    let _ : () = conn.set(remaining_key!(pie.id), pie.slices).unwrap();

    let n : u64 = conn.get(remaining_key!(pie.id)).unwrap();
    n
}