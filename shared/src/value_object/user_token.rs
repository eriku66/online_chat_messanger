use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserToken {
    value: String,
}

impl Default for UserToken {
    fn default() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
        }
    }
}

impl UserToken {
    pub fn value(&self) -> &str {
        &self.value
    }
}
