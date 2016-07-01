
extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Pies {
    pub pies: Vec<Pie>
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Pie {
    pub id: u64,
    pub name: String,
    pub image_url: String,
    pub price_per_slice: f64,
    pub slices: u64,
    pub labels: Vec<String>
}

pub fn new(json: String) -> Vec<Pie> {
    let decoded: Pies = json::decode(&json).unwrap();
    println!("{:?}", decoded.pies);
    decoded.pies
}