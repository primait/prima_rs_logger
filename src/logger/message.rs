use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slog::Key;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    pub app: &'a str,
    pub message: String,
    #[serde(borrow)]
    pub metadata: Fields<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct Fields<'a> {
    pub target: &'a str,
    pub level: &'a str,
    pub file: &'a str,
    pub line: u32,
    pub environment: String,
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}

pub struct AdditionalFields(pub HashMap<String, String>);

impl slog::Serializer for AdditionalFields {
    fn emit_arguments(&mut self, key: Key, val: &std::fmt::Arguments) -> slog::Result {
        self.0.insert(String::from(key), format!("{}", val));
        Ok(())
    }
}
