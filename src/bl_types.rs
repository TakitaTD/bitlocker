use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    pub password: String,
    pub platform: String,
    pub username: String,
    pub id: String,
    pub chrono: u32,
}
impl Entry {
    pub fn new(password: &str, platform: &str, username: &str, id: Uuid) -> Self {
        Self {
            password: password.to_string(),
            platform: platform.to_string(),
            username: username.to_string(),
            id: id.to_string(),
            chrono: 0,
        }
    }
    pub fn defaults() -> Self {
        Self {
            password: "".to_string(),
            platform: "".to_string(),
            username: "".to_string(),
            id: Uuid::new_v4().to_string(),
            chrono: 0,
        }
    }
}
