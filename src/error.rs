
use serde::Serialize;
use warp::reject;

quick_error! {
    #[derive(Debug)]
    pub enum HttpError {
        ClientError {}
        ServerError {}
    }
}

#[derive(Debug)]
pub struct ServerError;

impl reject::Reject for ServerError {}