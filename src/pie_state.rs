
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::thread;

use r2d2_redis::RedisConnectionManager;
use redis::Commands;

use std::collections::HashMap;
use std::ops::Deref;

extern crate bit_vec;
use bit_vec::BitVec;

use pies;

macro_rules! remaining_key { ($x:expr) => (format!("pie-{}-remaining", $x)) }
macro_rules! purchases_key { ($x:expr) => (format!("pie-{}-purchases", $x)) }
macro_rules! user_blacklist_key { ($x:expr) => (format!("user-{}-blacklist", $x)) }

pub fn set_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) {
    let conn = pool.get().expect("redis connection failed");
    let _ : () = conn.set(remaining_key!(pie.id), pie.slices).unwrap();

//    let n : u64 = conn.get(remaining_key!(pie.id)).unwrap();
//    println!("setting remaining for pie {} to {}", pie.name, n);
}

pub fn get_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, pie: &pies::Pie) -> u64 {
    let conn = pool.get().expect("redis connection failed");
    let n : u64 = conn.get(remaining_key!(pie.id)).unwrap();
    n
}

pub fn get_all_remaining(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, ids: &Vec<&u64>) -> Vec<u64> {
    let conn = pool.get().expect("redis connection failed");
    let keys : Vec<String> = ids.iter().map( |&id|
        remaining_key!(id)
    ).collect();
    let n : Vec<u64> = conn.get(keys).unwrap();
    n
}

pub enum PurchaseStatus {
    Fatty,
    Gone,
    Success
}

const ALLOWED_PIES: isize = 3;

pub fn user_blacklist(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>, user: &String) -> BitVec {
    let conn = pool.get().expect("redis connection failed");
    get_user_blacklist(&conn, user)
}

fn get_user_blacklist(conn: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, user: &String) -> BitVec {
    let bits : Vec<u8> = conn.get(user_blacklist_key!(user)).unwrap();
    let bitvec = BitVec::from_bytes(&bits);
    println!("{}: {:?}", user, bitvec);
    bitvec
}

fn set_user_blacklist(conn: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, user: &String, bitvec_pos: usize) {
    println!("{:?} {:?}", user, bitvec_pos);

    // this doesn't work for some reason, so using the raw command version
    // let bitset : bool = conn.setbit(, bitvec_pos, true).unwrap();
    let _ : () = redis::cmd("SETBIT")
        .arg(user_blacklist_key!(user))
        .arg(bitvec_pos)
        .arg(1)
        .query(conn.deref())
        .unwrap();

}

fn check_user_blacklist(conn: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, user: &String, bitvec_pos: usize) -> bool {
    let bitset : bool = conn.getbit(user_blacklist_key!(user), bitvec_pos).unwrap();
    bitset
}

pub fn purchase_pie(pool: &r2d2::Pool<r2d2_redis::RedisConnectionManager>,
                    pie: &pies::Pie,
                    bitvec_pos: usize,
                    user: &String,
                    amount: isize) -> PurchaseStatus {
    if amount > ALLOWED_PIES {
        return PurchaseStatus::Fatty;
    }

    let conn = pool.get().expect("redis connection failed");
    get_user_blacklist(&conn, user);
    if check_user_blacklist(&conn, user, bitvec_pos) {
//        println!("blocked purchase via blacklist");
        return PurchaseStatus::Fatty;
    }

    let prev_purchase : bool = conn.hexists(purchases_key!(pie.id), user).unwrap();
    let num_left : isize = conn.get(remaining_key!(pie.id)).unwrap();

    if num_left <= 0 {
        return PurchaseStatus::Gone;
    }

    if prev_purchase {
        let previous_amount : isize = conn.hget(purchases_key!(pie.id), user).unwrap();
//        println!("previous amount {:?}", previous_amount);
        if previous_amount + amount > ALLOWED_PIES {
            return PurchaseStatus::Fatty;
        } else {
            if amount > num_left {
                return PurchaseStatus::Gone;
            }

            if previous_amount + amount == ALLOWED_PIES {
//                println!("reached max amount");
                set_user_blacklist(&conn, user, bitvec_pos)
            }

            let n : isize = conn.hincr(purchases_key!(pie.id), user, amount).unwrap();
            let _ : () = conn.incr(remaining_key!(pie.id), -1 * amount).unwrap();
//            println!("bought {} pies total!", n)
        }
    } else {
//        println!("buying pie!");
        if amount == ALLOWED_PIES {
            set_user_blacklist(&conn, user, bitvec_pos)
        }

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

//    println!("{:?}", vec);
    vec
}