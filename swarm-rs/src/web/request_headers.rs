use std::collections::HashMap;

use rocket::{
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestHeaders {
    pub headers: HashMap<String, Vec<String>>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let mut headers = HashMap::new();

        // Iterate through the headers and collect them
        for header in request.headers().iter() {
            headers
                .entry(header.name().to_string())
                .or_insert_with(Vec::new)
                .push(header.value().to_string());
        }
        Outcome::Success(Self {
            headers
        })
    }
}
