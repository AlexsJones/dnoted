
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Struct to store and retrieve
#[derive(Serialize, Deserialize)]
pub struct Configuration {

    pub data: HashMap<String, String>, 
}