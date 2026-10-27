use axum::http::StatusCode;
use hello_loco::app::App;
use loco_rs::testing::prelude::*;
use serde_json::{json, Value};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn comments_belong_to_their_post() {
    request::<App, _, _>(|request, _ctx| async move {
        let first = request
            .post("/api/posts")
            .json(&json!({"title":"Tour post","content":"hi"}))
            .await;
        first.assert_status(StatusCode::CREATED);
        let first: Value = first.json();
        let id = first["id"].as_i64().unwrap();
        let second = request
            .post("/api/posts")
            .json(&json!({"title":"Other post"}))
            .await;
        second.assert_status(StatusCode::CREATED);
        let second: Value = second.json();
        let other_id = second["id"].as_i64().unwrap();

        let comment = request
            .post("/api/comments")
            .json(&json!({"content":"nice post","post_id":id}))
            .await;
        comment.assert_status(StatusCode::CREATED);
        let comment: Value = comment.json();
        assert_eq!(comment["post_id"], id);
        let comments = request.get(&format!("/api/posts/{id}/comments")).await;
        comments.assert_status_ok();
        let comments: Vec<Value> = comments.json();
        assert_eq!(comments, vec![comment]);
        let empty = request
            .get(&format!("/api/posts/{other_id}/comments"))
            .await;
        empty.assert_status_ok();
        assert!(empty.json::<Vec<Value>>().is_empty());
        request
            .get("/api/posts/9223372036854775807/comments")
            .await
            .assert_status_not_found();
        request
            .post("/api/comments")
            .json(&json!({"post_id":9223372036854775807_i64}))
            .await
            .assert_status_not_found();
        assert_eq!(request.get("/api/comments").await.status_code(), 405);
        request
            .post("/api/comments")
            .json(&json!({"content":"missing post"}))
            .await
            .assert_status_unprocessable_entity();
    })
    .await;
}
