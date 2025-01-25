use serde::{Deserialize, Serialize};

use super::{passwd::{hash_password, verify_password}, token::create_token, NewUser};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub login: String,
    pub email: Option<String>,
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

    pub fn from_new_user(new_user: NewUser) -> Self {
        Self {
            id: new_user.login.to_string(),
            login: new_user.login,
            full_name: new_user.full_name,
            email: new_user.email,
            password: hash_password(&new_user.password),
            roles: new_user.roles,
        }
    }
}

