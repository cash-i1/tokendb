use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    token: u64,
    pub username: String,
    pub balance: f32,
}
impl User {
    pub fn new(username: String, password: String) -> User {
        let token =
            (username.chars().map(|c| c as u64).sum::<u64>() * password.len() as u64) << 2 * 4 + 42;
        User {
            token,
            username,
            balance: 0.0,
        }
    }
    pub fn token(&self) -> u64 {
        self.token
    }
}
