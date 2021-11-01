use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Entity {
    pub message: String,
    pub number: u8,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            message: "Default Message".to_string(),
            number: 13,
        }
    }
}
