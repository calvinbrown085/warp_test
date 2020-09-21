

use crate::error::HttpError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DatabaseResult {
    pub value: String,
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseQuery {
    pub id: u32
}

pub trait DoSomethingWithDatabase {
    fn select_data(&self) -> Result<DatabaseResult, HttpError>;
}

impl DoSomethingWithDatabase for DatabaseQuery {
    fn select_data(&self) -> Result<DatabaseResult, HttpError> {
        Err(HttpError::ServerError)
    }
}


