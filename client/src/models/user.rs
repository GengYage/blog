use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserPreview {
    // github user id
    pub id: Option<u64>,
    /// github user name
    pub name: Option<String>,
    /// github home page
    pub html_url: Option<String>,
    /// avatar url
    pub avatar_url: Option<String>,
}
