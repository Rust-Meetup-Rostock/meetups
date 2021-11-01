use super::error::Error;
use std::convert::Infallible;
use warp::{hyper::StatusCode, reply::with_status, Rejection, Reply};

pub async fn recover(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let (status, message) = if rejection.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(error) = rejection.find::<Error>() {
        match error {
            Error::CustomError1 => (StatusCode::NOT_FOUND, error.to_string()),
            Error::CustomError2 => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        }
    } else if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(with_status(message, status))
}
