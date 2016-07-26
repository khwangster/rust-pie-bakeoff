extern crate iron;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;

extern crate router;
extern crate core;

pub fn not_found() -> IronResult<Response> {
    Ok(Response::with((
                          status::NotFound,
                          "Not Found",
                          Header(ContentType::plaintext())
                      )))
}

pub fn error() -> IronResult<Response> {
    Ok(Response::with((
                          status::InternalServerError,
                          "Internal Server Error",
                          Header(ContentType::plaintext())
                      )))
}

pub fn gone() -> IronResult<Response> {
    Ok(Response::with((
                          status::Gone,
                          "{\"error\": \"No more of that pie.  Try something else.\"}",
                          Header(ContentType::json())
                      )))
}

pub fn purchased() -> IronResult<Response> {
    Ok(Response::with((
                          status::Created,
                          "{\"text\": \"You bought some pie.\"}",
                          Header(ContentType::json())
                      )))
}

pub fn glutton() -> IronResult<Response> {
    Ok(Response::with((
                          status::TooManyRequests,
                          "{\"error\": \"Gluttony is discouraged.\"}",
                          Header(ContentType::json())
                      )))
}

pub fn no_recommends() -> IronResult<Response> {
    Ok(Response::with((
                          status::NotFound,
                          "{\"error\": \"Sorry we don’t have what you’re looking for.  Come back early tomorrow before the crowds come from the best pie selection.\"}",
                          Header(ContentType::json())
                      )))
}

pub fn recommend(i: usize) -> IronResult<Response> {
    Ok(Response::with((
                          status::Ok,
                          format!("{{ \"pie_url\": \"http://rust.fanboy.app/pies/{}\" }}", i),
                          Header(ContentType::json())
                      )))
}

pub fn bad_math() -> IronResult<Response> {
    Ok(Response::with((
                          status::PaymentRequired,
                          "{\"error\": \"You did math wrong.\"}",
                          Header(ContentType::json())
                      )))
}

//pub fn debug<T>(something: T) -> IronResult<Response>
//    where T: core::fmt::Debug {
//    Ok(Response::with((
//                          status::Ok,
//                          format!("{:?}", something),
//                          Header(ContentType::plaintext())
//                      )))
//}

pub fn text(string: String) -> IronResult<Response> {
    Ok(Response::with((
                          status::Ok,
                          string,
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