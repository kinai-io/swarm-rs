use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{
    agent::{Action, Output},
    prelude::*, utils::json_io,
};

use super::{verify_token, UserInfo};


#[derive(Serialize, Deserialize)]
pub struct AuthAgent {
    db_path: String,
    server_secret: String,
    token_validity_in_days: i64,
    protected_actions: HashMap<String, Vec<String>>,
}

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
        let users: Vec<UserInfo> = json_io::load(&self.db_path).unwrap();
        let existing_user = users.into_iter().find(|u| &u.login == login);
        existing_user
    }

    pub fn find_user_from_id(&self, id: &str) -> Option<UserInfo> {
        let users: Vec<UserInfo> = json_io::load(&self.db_path).unwrap();
        let existing_user = users.into_iter().find(|u| u.id == id);
        existing_user
    }

    pub fn login(&self, credentials: UserCredentials) -> Result<UserAuth, ()> {
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
        Err(())
    }

    pub fn refresh_token(&self, token: &str) -> Result<UserAuth, ()> {
        if let Ok(auth) = self.check_token(token) {
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
        Err(())
    }

    pub fn check_token(&self, token: &str) -> Result<AuthInfo, ()> {
        match verify_token(token, &self.server_secret) {
            Ok(claims) => Ok(AuthInfo {
                user_id: claims.user_id,
                roles: claims.roles,
            }),
            Err(_) => Err(()),
        }
    }

    pub fn is_accessible(&self, token: &str, action: &str) -> bool {
        if let Some(roles) = self.protected_actions.get(action) {
            if let Ok(auth_info) = self.check_token(token) {
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

#[async_trait]
impl Agent for AuthAgent {

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self, _action: &Action, _swarm: &Swarm) -> Output {
        Output::new_success(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    user_id: String,
    full_name: String,
    token: String,
    roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthInfo {
    pub user_id: String,
    pub roles: Vec<String>,
}
