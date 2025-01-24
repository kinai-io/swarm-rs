use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: String,
    pub roles: Vec<String>,
    pub exp: usize,
}


pub fn create_token(user_id: &str, roles: &Vec<String>, secret: &str, validity_in_hours: i64) -> String {

    let validity_duration = chrono::Duration::hours(validity_in_hours);
    // let validity_duration = chrono::Duration::seconds(10);
    let expiration = chrono::Utc::now()
        .checked_add_signed(validity_duration)
        .expect("valid timestamp")
        .timestamp();

    let claims = TokenClaims {
        user_id: user_id.to_owned(),
        roles: roles.clone(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn verify_token(token: &str, secret: &str) -> Result<TokenClaims, String> {
    let validation = Validation::default();
    let claims = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );
    match claims {
        Ok(claims) => Ok(claims.claims),
        Err(_) => Err(String::from("Invalid token"))
    }
    
}

