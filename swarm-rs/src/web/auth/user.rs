use serde::{Deserialize, Serialize};

use super::{passwd::verify_password, token::create_token};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub login: String,
    pub full_name: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl UserInfo {
    pub fn is_password_valid(&self, password: &str) -> bool {
        verify_password(password, &self.password)
    }

    pub fn new_token(&self, secret: &str, validity_in_hours: i64) -> String {
        create_token(&self.login, &self.roles, secret, validity_in_hours)
    }
}

