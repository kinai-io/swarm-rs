use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{
    agent::{Action, Output},
    prelude::*,
    utils::json_io,
};

use super::{passwd::hash_password, verify_token, UserInfo};

#[derive(Serialize, Deserialize)]
pub struct AuthAgent {
    db_path: String,
    server_secret: String,
    token_validity_in_days: i64,
    protected_actions: HashMap<String, Vec<String>>,
}

#[agent]
impl AuthAgent {
    pub fn new(db_path: &str, server_secret: &str, token_validity_in_days: i64) -> Self {
        Self {
            db_path: db_path.to_string(),
            server_secret: server_secret.to_string(),
            token_validity_in_days,
            protected_actions: HashMap::new(),
        }
    }

    pub fn find_user_from_login(&self, login: &str) -> Option<UserInfo> {
        let users: Vec<UserInfo> = self.load_users();
        let existing_user = users.into_iter().find(|u| &u.login == login);
        existing_user
    }

    pub fn find_user_from_id(&self, id: &str) -> Option<UserInfo> {
        let users: Vec<UserInfo> = self.load_users();
        let existing_user = users.into_iter().find(|u| u.id == id);
        existing_user
    }

    fn load_users(&self) -> Vec<UserInfo> {
        if let Ok(users) = json_io::load(&self.db_path) {
            users
        }else {
            let users = vec![];
            json_io::write(&self.db_path, &users).unwrap();
            users
        }
    }

    pub fn add_user(&self, user_info: &UserInfo) {
        let mut users: Vec<UserInfo> = self.load_users();
        users.push(user_info.clone());
        json_io::write(&self.db_path, &users).unwrap();
    }

    pub fn save_user(&self, user_info: &UserInfo) {
        let users: Vec<UserInfo> = self.load_users();
        let users: Vec<UserInfo> = users.into_iter().map(|u| {
            if u.id == user_info.id {
                user_info.clone()
            }else {
                u
            }
        }).collect();
        json_io::write(&self.db_path, &users).unwrap();
    }

    #[agent_action]
    pub async fn register_user(&self, new_user: NewUser) -> Result<UserAuth, String> {
        if let Some(_user) = self.find_user_from_login(&new_user.login) {
            Err("Unable to add User".to_string())
        } else {
            let user_info = UserInfo::from_new_user(new_user);
            self.add_user(&user_info);
            Ok(UserAuth {
                user_id: user_info.id.to_string(),
                full_name: user_info.full_name.to_string(),
                roles: user_info.roles.clone(),
                token: user_info.new_token(&self.server_secret, self.token_validity_in_days * 24),
            })
        }
    }

    #[agent_action]
    pub async fn login(&self, credentials: UserCredentials) -> Result<UserAuth, String> {
        let existing_user = self.find_user_from_login(&credentials.login);

        match existing_user {
            Some(user) => {
                if user.is_password_valid(&credentials.password) {
                    let secret = &self.server_secret;
                    let validity_in_hours = self.token_validity_in_days * 24;
                    let token = user.new_token(secret, validity_in_hours);
                    return Ok(UserAuth {
                        user_id: user.id.to_string(),
                        full_name: user.full_name.to_string(),
                        roles: user.roles.clone(),
                        token,
                    });
                }
            }
            _ => {}
        }
        Err("Auth error".to_string())
    }

    #[agent_action]
    pub async fn refresh_token(&self, token: UserToken) -> Result<UserAuth, String> {
        if let Ok(auth) = token.check_token(&self.server_secret) {
            let existing_user = self.find_user_from_id(&auth.user_id);
            match existing_user {
                Some(user) => {
                    let secret = &self.server_secret;
                    let validity_in_hours = self.token_validity_in_days * 24;
                    let token = user.new_token(secret, validity_in_hours);
                    return Ok(UserAuth {
                        user_id: user.id.to_string(),
                        full_name: user.full_name.to_string(),
                        roles: user.roles.clone(),
                        token,
                    });
                }
                _ => {}
            }
        }
        Err("Auth error".to_string())
    }

    #[agent_action]
    pub async fn update_password(&self, update: PasswordUpdate) -> Result<UserAuth, String> {
        let token = UserToken::new(&update.token);
        if let Ok(auth) = token.check_token(&self.server_secret) {
            let existing_user = self.find_user_from_id(&auth.user_id);
            match existing_user {
                Some(mut user) => {
                    user.password = hash_password(&update.new_password);
                    self.save_user(&user);
                    let secret = &self.server_secret;
                    let validity_in_hours = self.token_validity_in_days * 24;
                    let token = user.new_token(secret, validity_in_hours);
                    return Ok(UserAuth {
                        user_id: user.id.to_string(),
                        full_name: user.full_name.to_string(),
                        roles: user.roles.clone(),
                        token,
                    });
                }
                _ => {}
            }
        }
        Err("Auth error".to_string())
    }
    

    pub fn is_accessible(&self, token: &str, action: &str) -> bool {
        if let Some(roles) = self.protected_actions.get(action) {
            let token = UserToken::new(token);
            if let Ok(auth_info) = token.check_token(&self.server_secret) {
                for role in roles {
                    if auth_info.roles.contains(role) {
                        return true;
                    }
                }
            }
            false
        } else {
            true
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub token: String,
}

impl UserToken {

    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string()
        }
    }

    pub fn check_token(&self, server_secret: &str) -> Result<AuthInfo, ()> {
        match verify_token(&self.token, server_secret) {
            Ok(claims) => Ok(AuthInfo {
                user_id: claims.user_id,
                roles: claims.roles,
            }),
            Err(_) => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    pub user_id: String,
    pub full_name: String,
    pub token: String,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthInfo {
    pub user_id: String,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub login: String,
    pub password: String,
    pub email: Option<String>,
    pub full_name: String,
    pub roles: Vec<String>,
}

impl NewUser {
    pub fn new(login: &str, password: &str, full_name: &str, email: &str, roles: Vec<&str>) -> Self {
        let email = if email.len() > 0 {
            Some(email.to_string())
        }else {
            None
        };
        Self {
            login: login.to_string(),
            password: password.to_string(),
            email,
            full_name: full_name.to_string(),
            roles: roles.iter().map(|r| r.to_string()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PasswordUpdate {
    pub token: String,
    pub old_password: String,
    pub new_password: String
}
