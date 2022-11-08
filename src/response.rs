use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>,
}

impl<T> Response<T> {
    fn new() -> Self {
        Self { results: vec![] }
    }
}

impl<T> Default for Response<T> {
    fn default() -> Self {
        Self::new()
    }
}