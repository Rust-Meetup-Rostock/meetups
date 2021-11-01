use warp::{Filter, Rejection, Reply};

use super::handlers;
use super::State;

pub fn get_routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    add_entity(state.clone())
        .or(get_all_entities(state.clone()))
        .or(get_entity(state.clone()))
        .or(delete_entity(state))
}

fn add_entity(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::any().map(move || state.clone()))
        .and(warp::path("state"))
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(handlers::add_entity)
}

fn get_entity(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::any().map(move || state.clone()))
        .and(warp::path("state"))
        .and(warp::path::param::<u8>())
        .and(warp::path::end())
        .and_then(handlers::get_entity)
}

fn get_all_entities(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::any().map(move || state.clone()))
        .and(warp::path("state"))
        .and(warp::path::end())
        .and_then(handlers::get_all_entities)
}

fn delete_entity(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(warp::any().map(move || state.clone()))
        .and(warp::path("state"))
        .and(warp::path::param::<u8>())
        .and(warp::path::end())
        .and_then(handlers::delete_entity)
}
