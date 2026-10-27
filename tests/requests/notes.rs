use axum::http::StatusCode;
use hello_loco::{
    app::App,
    models::{notes, users},
    views::auth::LoginResponse,
};
use loco_rs::testing::prelude::*;
use sea_orm::{EntityTrait, ModelTrait, PaginatorTrait};
use serde_json::{json, Value};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn auth_tutorial_register_login_and_create_note() {
    request::<App, _, _>(|request, ctx| async move {
        let payload = json!({"title":"secret", "content":"shh"});
        request
            .get("/api/auth/current")
            .await
            .assert_status_unauthorized();
        request
            .get("/api/auth/current")
            .add_header("Authorization", "Bearer invalid")
            .await
            .assert_status_unauthorized();
        request
            .post("/api/notes")
            .json(&payload)
            .await
            .assert_status_unauthorized();
        request
            .post("/api/notes")
            .add_header("Authorization", "Bearer invalid")
            .json(&payload)
            .await
            .assert_status_unauthorized();
        assert_eq!(notes::Entity::find().count(&ctx.db).await.unwrap(), 0);

        request
            .post("/api/auth/register")
            .json(&json!({
                "name":"Loco user", "email":"user@loco.rs", "password":"12341234"
            }))
            .await
            .assert_status_ok();
        let user = users::Model::find_by_email(&ctx.db, "user@loco.rs")
            .await
            .unwrap();
        assert!(user.email_verified_at.is_none());
        assert_eq!(ctx.mailer.as_ref().unwrap().deliveries().count, 1);
        let login = request
            .post("/api/auth/login")
            .json(&json!({
                "email":"user@loco.rs", "password":"12341234"
            }))
            .await;
        login.assert_status_ok();
        let login: LoginResponse = login.json();
        assert!(!login.is_verified);
        let bearer = format!("Bearer {}", login.token);
        let current = request
            .get("/api/auth/current")
            .add_header("Authorization", &bearer)
            .await;
        current.assert_status_ok();
        let current: Value = current.json();
        assert_eq!(
            current,
            json!({"pid":login.pid,"name":"Loco user","email":"user@loco.rs"})
        );

        let created = request
            .post("/api/notes")
            .add_header("Authorization", &bearer)
            .json(&payload)
            .await;
        created.assert_status(StatusCode::CREATED);
        let note: Value = created.json();
        assert_eq!(note["title"], "secret");
        assert_eq!(note["content"], "shh");
        assert_eq!(notes::Entity::find().count(&ctx.db).await.unwrap(), 1);
        let path = format!("/api/notes/{}", note["id"]);

        // The tutorial only protects creation; the other CRUD routes stay public.
        let list = request.get("/api/notes").await;
        list.assert_status_ok();
        assert_eq!(list.json::<Value>()["items"][0], note);
        request.get(&path).await.assert_status_ok();
        let updated = request
            .put(&path)
            .json(&json!({"title":"updated","content":null}))
            .await;
        updated.assert_status_ok();
        assert_eq!(updated.json::<Value>()["title"], "updated");
        request
            .delete(&path)
            .await
            .assert_status(StatusCode::NO_CONTENT);
        request.get(&path).await.assert_status_not_found();

        // A correctly signed JWT is insufficient if its user no longer exists.
        user.delete(&ctx.db).await.unwrap();
        request
            .post("/api/notes")
            .add_header("Authorization", &bearer)
            .json(&payload)
            .await
            .assert_status_not_found();
        assert_eq!(notes::Entity::find().count(&ctx.db).await.unwrap(), 0);
    })
    .await;
}
