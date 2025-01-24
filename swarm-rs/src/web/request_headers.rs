use rocket::{
    outcome::Outcome,
    request::{self, FromRequest, Request},
};

pub struct RequestHeaders {
    pub token: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(Self {
            token: request.headers().get_one("Token").unwrap_or("").to_string()
        })
    }
}