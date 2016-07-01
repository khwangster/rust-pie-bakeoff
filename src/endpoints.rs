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

use response;
use pies;
use cache;

pub fn hello_world(req: &mut Request) -> IronResult<Response> {
    let pies = req.get::<Read<cache::LabelIndex>>().unwrap();
    response::debug(pies)
}

pub fn pies(req: &mut Request) -> IronResult<Response> {
    response::json(format!("{{ \"json\": \"{}\" }}", req.url))
}

pub fn pie(req: &mut Request) -> IronResult<Response> {
    let pie_option = req.extensions.get::<Router>()
        .unwrap()
        .find("pie_id");

    match pie_option {
        Some(x) => {
            let pie_id = x.trim_right_matches(".json");
            match req.url.path.last() {
                Some(x) if x.ends_with("json") => {
                    response::json(format!("{{ \"json\": \"{}\" }}", pie_id))
                }
                Some(x) => {
                    response::html(format!("<html><h1>{}</h1></html>", pie_id))
                }
                _ => response::not_found()
            }
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