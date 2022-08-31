use std::str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemBody<T> {
    pub item: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewItem {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    pub catalog: String,
}
