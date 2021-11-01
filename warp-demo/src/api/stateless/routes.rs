use warp::http::{HeaderMap, StatusCode};
use warp::reply::{json, with_status};
use warp::{Filter, Rejection, Reply};

use crate::api::error::Error;

use super::list_headers;
use super::Entity;

pub fn post_json() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("send-entity"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|body: Entity| with_status(json(&body), StatusCode::OK))
}

pub fn hello_world() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello" / String).map(|name| format!("Hello, {}!", name))
}

pub fn goodbye_world() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("goodbye"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .map(|name| format!("Goodbye, {}!", name))
}

pub fn read_body() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("gimme-json"))
        .and(warp::path::param::<u8>())
        .and(warp::path::end())
        .map(|number| {
            let response = Entity {
                message: "This is the response object.".to_string(),
                number: number,
            };
            with_status(json(&response), StatusCode::OK)
        })
}

pub fn read_headers() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("headers"))
        .and(warp::header::headers_cloned())
        .and(warp::path::end())
        .map(|headers: HeaderMap| {
            let headers = list_headers(headers);
            with_status(json(&headers), StatusCode::OK)
        })
}

pub fn read_specific_header() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().and(
        warp::path("headers")
            .and(warp::path("auth"))
            .and(warp::header::<String>("Authorization"))
            .and(warp::path::end())
            .map(|value: String| {
                let secret = value["Bearer ".len()..value.len()].to_string();
                with_status(secret, StatusCode::OK)
            }),
    )
}
