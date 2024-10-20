use crate::user::User;

use std::path::Path;

use serde_json::json;
use serde_json::Value;

#[derive(Clone)]
pub struct Database {
    path: &'static Path,
    pub current_user: Option<User>,
}
impl Database {
    pub fn new(path: &'static Path) -> Database {
        if !path.is_file() {
            panic!()
        }
        if !Path::exists(path) {
            std::fs::write(path, "").unwrap();
        }
        Database {
            path,
            current_user: None,
        }
    }
    pub fn get_users(&self) -> Vec<User> {
        let string = std::fs::read_to_string(self.path).unwrap();
        let json: Value = serde_json::from_str(&string).unwrap();
        let users: Vec<User> = serde_json::from_value(json.get("users").unwrap().clone()).unwrap();
        users
    }
    pub fn get_user(&self, token: u64) -> Option<User> {
        let users = self.get_users();
        users.iter().cloned().find(|user| user.token() == token)
    }
    pub fn add_user_if_not_already_exists(&self, user: &User) {
        let string = std::fs::read_to_string(self.path).unwrap();
        let json: Value = serde_json::from_str(&string).unwrap();
        let mut users: Vec<User> =
            serde_json::from_value(json.get("users").unwrap().clone()).unwrap();
        if users
            .iter()
            .find(|u| u.token() == user.token() && u.username == user.username)
            .is_some()
        {
            return;
        }
        users.push(user.clone());
        let new_json: Value = serde_json::from_value(json!({"users": users})).unwrap();
        let new_string = serde_json::to_string_pretty(&new_json).unwrap();
        std::fs::write(self.path, new_string).unwrap();
    }
    pub fn update_user(&self, token: u64, new_user: &User) {
        let string = std::fs::read_to_string(self.path).unwrap();
        let json: Value = serde_json::from_str(&string).unwrap();
        let mut users: Vec<User> =
            serde_json::from_value(json.get("users").unwrap().clone()).unwrap();

        let pos = users.iter().position(|u| u.token() == token);
        if pos.is_none() {
            return;
        }
        let pos = pos.unwrap();

        users[pos] = new_user.clone();

        let new_json: Value = serde_json::from_value(json!({"users": users})).unwrap();
        let new_string = serde_json::to_string_pretty(&new_json).unwrap();
        std::fs::write(self.path, new_string).unwrap();
    }
}
