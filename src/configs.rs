use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub directories_to_clean: Vec<String>
}

impl Configs{
    pub fn new() -> Self {
        Self {
            directories_to_clean: Vec::new()
        }
    }
}