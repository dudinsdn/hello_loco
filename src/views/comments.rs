use crate::models::comments;
use loco_rs::prelude::DateTimeWithTimeZone;
use serde::Serialize;

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i64,
    pub content: Option<String>,
    pub post_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<comments::Model> for CommentResponse {
    fn from(model: comments::Model) -> Self {
        Self {
            id: model.id,
            content: model.content,
            post_id: model.post_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
