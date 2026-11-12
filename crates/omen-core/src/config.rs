use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub performance: String,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            performance: "Balanced".into(),
            theme: "dark".into(),
        }
    }
}
