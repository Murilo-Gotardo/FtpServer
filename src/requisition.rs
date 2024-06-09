use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requisition<'a> {
    command: String,
    file_name: Option<String>,
    file_data: Option<&'a[u8]>,
    hash: Option<String>
}

impl<'a> Requisition<'a> {
    pub fn new(command: String, file_name: Option<String>, file_data: Option<&'a[u8]>, hash: Option<String>) -> Self {
        Self { command, file_name, file_data, hash }
    }
    
    pub fn command(&self) -> &str {
        &self.command
    }
    pub fn file_name(&self) -> &Option<String> {
        &self.file_name
    }
    pub fn file_data(&self) -> Option<&'a [u8]> {
        self.file_data
    }
    pub fn hash(&self) -> &Option<String> {
        &self.hash
    }
}