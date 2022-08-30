use serde::{Deserialize, Serialize};

use super::request::ItemJson;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScItem {
    pub url: String,
    pub title: String,
    pub tags: String,
    pub catalog: String,
}

impl From<ItemJson> for ScItem {
    fn from(frm: ItemJson) -> Self {
        let tag_string = frm
            .tags
            .iter()
            .fold(String::new(), |tag_str, t| format!("{},{}", tag_str, t));
        let tag_string = tag_string.strip_prefix(',').unwrap().to_string();
        Self {
            url: frm.url,
            title: frm.title,
            tags: tag_string,
            catalog: frm.catalog,
        }
    }
}
