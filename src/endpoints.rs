extern crate iron;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;

extern crate router;
use router::Router;

extern crate url;
use url::{Url, Host};

extern crate persistent;
use persistent::{State, Read, Write};
use iron::typemap::Key;

extern crate rustc_serialize;
use rustc_serialize::json;

use std::str::FromStr;
use std::str;

extern crate mustache;
use mustache::{MapBuilder, Template};

use response;
use pies;
use pie_state;
use cache;

pub fn hello_world(req: &mut Request) -> IronResult<Response> {
    let pies = req.get::<Read<cache::LabelIndex>>().unwrap();
    response::debug(pies)
}

pub fn pies(req: &mut Request) -> IronResult<Response> {
//    response::json(format!("{{ \"json\": \"{}\" }}", req.url));

//    let url = Url::parse(req.url).unwrap_or_else(
//        { return response::error(); }
//    );
    let url = req.url.clone().into_generic_url();
    match url.query_pairs() {
        Some(vec) => {

        },
        None => return response::error()
    };

    response::debug(&url.query_pairs())
}

fn pie_template() -> mustache::Template {
    mustache::compile_str("
    {{#pies}}
    <h1>{{name}}</h1>
    <img src=\"{{image_url}}\" width=\"50%\"></img>
    <p>price: {{price_per_slice}}</p>
    <p>remaining: {{remaining_slices}}</p>
    <p> {{#purchases}}
        <p>{{username}} purchased {{slices}}<p>
        {{/purchases}}
    </p>
    {{/pies}}
    ")
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
        name: pie.name.clone(),
        image_url: pie.image_url.clone(),
        price_per_slice: pie.price_per_slice.clone(),
        remaining_slices: remaining,
        purchases: pie_state::pie_purchases(&redis, &pie)
    };

    match url_end {
        Some(x) if x.ends_with("json") => {
            let data: String = json::encode(&show_pie).unwrap();
            response::json(data)
        },
        Some(x) => {
            let mut bytes = vec![];
            let mut pies = vec![];
            pies.push(show_pie);
            pie_template().render(&mut bytes, &pies::ShowPies { pies: pies }).unwrap();

            response::html(format!("<html>{}</html>", str::from_utf8(&bytes).unwrap()))
        },
        _ => response::not_found()
    }
}

pub fn purchase(req: &mut Request) -> IronResult<Response> {
    let id_index = req.get::<Read<cache::IdIndex>>().unwrap();
    let redis = req.get::<Read<cache::Redis>>().unwrap();

    let extensions = req.extensions.get::<Router>()
        .unwrap();

    // return if we can't find pie_id
    let pie_id = u64::from_str(extensions.find("pie_id").unwrap()).unwrap();

    let pie = if let Some(x) = id_index.get(&pie_id) {
        x.clone()
    } else {
        return response::not_found()
    };

    let mut username = None;
    let mut amount = None;
    let mut slices = Some(1);

    let url = req.url.clone().into_generic_url();
    match url.query_pairs() {
        Some(vec) => {
            for &(ref name, ref value) in vec.iter() {
                match name.trim() {
                    "username" => {
                        username = Some(value.clone());
                    },
                    "amount" => {
                        amount = f64::from_str(&value.clone()).ok();
                    },
                    "slices" => {
                        slices = i64::from_str(&value.clone()).ok();
                    }
                    _ => {}
                }
            }
        },
        None => return response::error()
    };

    match (username, amount, slices) {
        (Some(u), Some(a), Some(s)) => {
            let price = pie.price_per_slice * s as f64;

            if (price - a).abs() > 1e-5 {
                response::bad_math()
            } else {
                match pie_state::purchase_pie(&redis, &pie, &u, s as isize) {
                    pie_state::PurchaseStatus::Success => {
                        response::purchased()

                    }
                    pie_state::PurchaseStatus::Fatty => {
                        response::glutton()

                    }
                    pie_state::PurchaseStatus::Gone => {
                        response::gone()

                    }
                }
            }
        },
        (Some(u), None, _) => {
            response::bad_math()
        },
        (_, _, _) => {
            response::error()
        }
    }

}

pub fn recommend(req: &mut Request) -> IronResult<Response> {
    response::text(String::from("hello"))
}