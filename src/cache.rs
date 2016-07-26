
extern crate persistent;
use persistent::State;
use iron::typemap::Key;
use std::collections::HashMap;
use std::collections::VecDeque;

extern crate bit_vec;
use bit_vec::BitVec;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
use r2d2_redis::RedisConnectionManager;

use pies;

#[derive(Copy, Clone)]
pub struct Redis;
impl Key for Redis { type Value = r2d2::Pool<r2d2_redis::RedisConnectionManager>; }

#[derive(Copy, Clone)]
pub struct SortedPies;
impl Key for SortedPies { type Value = VecDeque<pies::Pie>; }

#[derive(Copy, Clone)]
pub struct AllPieId;
impl Key for AllPieId { type Value = Vec<u64>; }

#[derive(Copy, Clone)]
pub struct IdIndex;
impl Key for IdIndex { type Value = HashMap<u64, (pies::Pie, usize)>; }

#[derive(Copy, Clone)]
pub struct LabelBitVec;
impl Key for LabelBitVec { type Value = HashMap<String, BitVec>; }

