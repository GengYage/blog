use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArticlePreview {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
