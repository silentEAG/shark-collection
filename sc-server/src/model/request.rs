use serde::Deserialize;

#[derive(Deserialize)]
pub struct UrlJson {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    pub catalog: String,
}
