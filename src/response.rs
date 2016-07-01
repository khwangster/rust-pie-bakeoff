extern crate iron;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;

extern crate router;
use router::Router;

extern crate core;
use self::core::fmt;

pub fn not_found() -> IronResult<Response> {
    Ok(Response::with((
                          status::NotFound,
                          "Not Found",
                          Header(ContentType::plaintext())
                      )))
}

pub fn debug<T>(something: T) -> IronResult<Response>
    where T: core::fmt::Debug {
    Ok(Response::with((
                          status::Ok,
                          format!("{:?}", something),
                          Header(ContentType::plaintext())
                      )))
}

pub fn text(string: String) -> IronResult<Response> {
    Ok(Response::with((
                          status::Ok,
                          format!("hello world {}", string),
                          Header(ContentType::plaintext())
                      )))
}

pub fn json(json: String) -> IronResult<Response> {
    Ok(Response::with((
                          status::Ok,
                          json,
                          Header(ContentType::json())
                      )))
}

pub fn html(html: String) -> IronResult<Response> {
    Ok(Response::with((
                          status::Ok,
                          html,
                          Header(ContentType::html())
                      )))
}