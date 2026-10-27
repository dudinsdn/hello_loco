use hello_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn server_side_home_and_static_assets_are_available() {
    request::<App, _, _>(|request, _ctx| async move {
        let home = request.get("/").await;
        home.assert_status_ok();
        let html = home.text();
        assert!(home.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/html"));
        assert!(html.contains("<h1>Hello, Loco.</h1>"));
        assert!(html.contains("/static/app.css"));
        assert!(!html.contains("{{ title }}"));
        assert!(html.contains("href=\"/api/notes\""));

        let css = request.get("/static/app.css").await;
        css.assert_status_ok();
        assert!(css.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/css"));
        assert!(css.text().contains(".resources"));
        request.get("/api/notes").await.assert_status_ok();
        request.get("/not-a-route").await.assert_status_not_found();
    })
    .await;
}
