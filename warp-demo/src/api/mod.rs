mod entities;
mod error;
mod recovery;
mod state;
mod stateful;
mod stateless;
mod utils;

use entities::Entity;
use state::State;
use stateless::*;
use utils::*;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

pub async fn run() {
    let state = State::default();
    let log = warp::log("api");

    let stateless_routes = get_stateless_routes();
    let stateful_routes = stateful::get_routes(state);
    let routes = stateless_routes.or(stateful_routes);
    let routes_with_recovery = routes.recover(recovery::recover);
    let routes_with_log = routes_with_recovery.with(log);

    warp::serve(routes_with_log)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn get_stateless_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    hello_world()
        .or(goodbye_world())
        .or(read_body())
        .or(read_headers())
        .or(read_specific_header())
        .or(post_json())
}
