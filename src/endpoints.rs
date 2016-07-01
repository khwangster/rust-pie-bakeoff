extern crate iron;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;

extern crate router;
use router::Router;

extern crate persistent;
use persistent::{State, Read, Write};
use iron::typemap::Key;

extern crate rustc_serialize;
use rustc_serialize::json;

use std::str::FromStr;

use response;
use pies;
use pie_state;
use cache;

pub fn hello_world(req: &mut Request) -> IronResult<Response> {
    let pies = req.get::<Read<cache::LabelIndex>>().unwrap();
    response::debug(pies)
}

pub fn pies(req: &mut Request) -> IronResult<Response> {
    response::json(format!("{{ \"json\": \"{}\" }}", req.url))
}

pub fn pie(req: &mut Request) -> IronResult<Response> {

    let id_index = req.get::<Read<cache::IdIndex>>().unwrap();
    let redis = req.get::<Read<cache::Redis>>().unwrap();
    let url_end = req.url.path.last();

    // req.extensions must go last because borrow checker is dumb
    let pie_option = req.extensions.get::<Router>()
        .unwrap()
        .find("pie_id");

    // return if we can't find pie_id
    let pie_id = if let Some(x) = pie_option {
        u64::from_str(x.trim_right_matches(".json")).unwrap()
    } else {
        return response::not_found()
    };

    // return if we can't find pie in cache
    let pie = if let Some(x) = id_index.get(&pie_id) {
        x.clone()
    } else {
        return response::not_found()
    };

    let remaining = pie_state::get_remaining(&redis, &pie);

    let show_pie = pies::ShowPie {
        name: pie.name,
        image_url: pie.image_url,
        price_per_slice: pie.price_per_slice,
        remaining_slices: remaining,
        purchases: vec![pies::Purchase {
                    username: "Ken".to_string(),
                    slices: 1u64
                }]
    };

    match url_end {
        Some(x) if x.ends_with("json") => {
            let data: String = json::encode(&show_pie).unwrap();
            response::json(data)
        },
        Some(x) => {
            response::html(format!("<html><h1>{}</h1></html>", pie_id))
        },
        _ => response::not_found()
    }
}

pub fn purchase(_: &mut Request) -> IronResult<Response> {
    response::text(String::from("hello"))
}

pub fn recommend(_: &mut Request) -> IronResult<Response> {
    response::text(String::from("hello"))
}