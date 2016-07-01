
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::thread;

use r2d2_redis::RedisConnectionManager;
use redis::Commands;

use std::collections::HashMap;

use pies;

macro_rules! remaining_key { ($x:expr) => (format!("pie-{}-remaining", $x)) }
macro_rules! purchases_key { ($x:expr) => (format!("pie-{}-purchases", $x)) }

pub fn set_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) {
    let conn = pool.get().expect("redis connection failed");
    let _ : () = conn.set(remaining_key!(pie.id), pie.slices).unwrap();

    let n : u64 = conn.get(remaining_key!(pie.id)).unwrap();
    println!("setting remaining for pie {} to {}", pie.name, n);
}

pub fn get_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) -> u64 {
    let conn = pool.get().expect("redis connection failed");
    let n : u64 = conn.get(remaining_key!(pie.id)).unwrap();
    n
}

//pub fn get_all_remaining(pool: pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, ids: Vec<u64>) {
//
//}

pub enum PurchaseStatus {
    Fatty,
    Gone,
    Success
}

pub fn purchase_pie(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie, user: &String, amount: isize) -> PurchaseStatus {
    if amount > 3 {
        return PurchaseStatus::Fatty;
    }

    let conn = pool.get().expect("redis connection failed");
    let exists : bool = conn.hexists(purchases_key!(pie.id), user).unwrap();
    let num_left : isize = conn.get(remaining_key!(pie.id)).unwrap();

    if num_left <= 0 {
        return PurchaseStatus::Gone;
    }

    if exists {
        let previous_amount : isize = conn.hget(purchases_key!(pie.id), user).unwrap();
        println!("previous amount {:?}", previous_amount);
        if previous_amount + amount > 3 {
            return PurchaseStatus::Fatty;
        } else {
            if num_left <= amount {
                return PurchaseStatus::Gone;
            }

            let n : isize = conn.hincr(purchases_key!(pie.id), user, amount).unwrap();
            let _ : () = conn.incr(remaining_key!(pie.id), -1 * amount).unwrap();
            println!("bought {} pies total!", n)
        }
    } else {
        println!("buying pie!");
        let n : isize = conn.hincr(purchases_key!(pie.id), user, amount).unwrap();
        let _ : () = conn.incr(remaining_key!(pie.id), -1 * amount).unwrap();
    }

    PurchaseStatus::Success
}

pub fn pie_purchases(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) -> Vec<pies::Purchase> {
    let conn = pool.get().expect("redis connection failed");

    let purchases : HashMap<String, u64> = conn.hgetall(purchases_key!(pie.id)).unwrap();

    let mut vec = Vec::new();
    for (user, amount) in &purchases {
        let purchase = pies::Purchase {
            username: user.clone(),
            slices: amount.clone()
        };
        vec.push(purchase);
    }

    println!("{:?}", vec);
    vec
}