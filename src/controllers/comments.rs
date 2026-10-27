use crate::views::comments::CommentResponse;
use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::models::comments;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub content: Option<String>,
    pub post_id: i64,
}

#[debug_handler]
async fn create(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let item = comments::Model::create_for_post(&ctx.db, params.post_id, params.content).await?;
    Ok((StatusCode::CREATED, Json(CommentResponse::from(item))).into_response())
}

pub fn routes() -> Routes {
    Routes::new().prefix("/api/comments").add("/", post(create))
}
