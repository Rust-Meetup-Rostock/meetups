use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Error, Serialize)]
pub enum Error {
    #[error("This is custom error 1.")]
    CustomError1,
    #[error("This is custom error 2.")]
    CustomError2,
}

impl warp::reject::Reject for Error {}
