use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemJson {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    pub catalog: String,
}
