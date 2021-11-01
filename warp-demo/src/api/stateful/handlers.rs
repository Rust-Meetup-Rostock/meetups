use super::Entity;
use super::State;
use std::convert::Infallible;
use warp::Reply;

pub async fn add_entity(state: State, entity: Entity) -> Result<impl Reply, Infallible> {
    match state.store.try_write() {
        Ok(mut store) => {
            let _ = store.insert(entity.number, entity.clone());
            Ok(warp::reply::with_status(
                warp::reply::json(&entity),
                warp::http::StatusCode::OK,
            ))
        }
        _ => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn get_entity(state: State, number: u8) -> Result<impl Reply, Infallible> {
    match state.store.try_read() {
        Ok(store) => match store.get(&number) {
            Some(entity) => Ok(warp::reply::with_status(
                warp::reply::json(&entity),
                warp::http::StatusCode::OK,
            )),
            None => Ok(warp::reply::with_status(
                warp::reply::json(&()),
                warp::http::StatusCode::NOT_FOUND,
            )),
        },
        _ => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn get_all_entities(state: State) -> Result<impl Reply, Infallible> {
    match state.store.try_read() {
        Ok(store) => {
            let values: Vec<Entity> = store.iter().map(|(_, value)| value.to_owned()).collect();
            Ok(warp::reply::with_status(
                warp::reply::json(&values),
                warp::http::StatusCode::OK,
            ))
        }
        _ => Ok(warp::reply::with_status(
            warp::reply::json(&(Vec::new() as Vec<Entity>)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn delete_entity(state: State, number: u8) -> Result<impl Reply, Infallible> {
    match state.store.try_write() {
        Ok(mut store) => match store.remove(&number) {
            Some(_) => Ok(warp::http::StatusCode::OK),
            None => Ok(warp::http::StatusCode::NOT_FOUND),
        },
        _ => Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
