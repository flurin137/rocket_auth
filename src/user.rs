use uuid::Uuid;

use crate::{auth::Credentials, hashing::*};

pub struct Database {
    pub users: Vec<User>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub hashed_password: String,
    user_id: Uuid
}

impl User {
    pub fn from_credentials(credentials: &Credentials) -> Self {
        let hashed_password = salt_password(&credentials.password);
        let username = credentials.username.to_string();
        let user_id = Uuid::new_v4();

        Self {
            username,
            hashed_password,
            user_id
        }
    }

    pub fn is_user_with_credentials(&self, credentials: &Credentials) -> bool {
        if credentials.username != self.username {
            return false;
        }

        return verify_password(&credentials.password, &self.hashed_password);
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    
}
