use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct PostItem {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub content: String,
    pub creator_id: String,
    pub deleted: bool,
    pub locked: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub enum Permission {
    Banned,
    Unverified,
    User,
    Admin,
    Root,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
    pub deleted: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}
