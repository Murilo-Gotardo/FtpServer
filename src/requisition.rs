use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requisition {
    command: String,
    file_name: Option<String>,
    hash: Option<String>
}

impl Requisition {
    pub fn new(command: String, file_name: Option<String>, hash: Option<String>) -> Self {
        Self { command, file_name, hash }
    }
    
    pub fn command(&self) -> &str {
        &self.command
    }
    pub fn file_name(&self) -> &Option<String> {
        &self.file_name
    }
    pub fn hash(&self) -> &Option<String> {
        &self.hash
    }
}

