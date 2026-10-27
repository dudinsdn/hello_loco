use hello_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn account_pages_and_script_are_served() {
    request::<App, _, _>(|request, _ctx| async move {
        let login = request.get("/login").await;
        login.assert_status_ok();
        assert!(login.text().contains("data-mode=\"login\""));
        assert!(login.text().contains("autocomplete=\"current-password\""));
        let register = request.get("/register").await;
        register.assert_status_ok();
        assert!(register.text().contains("data-mode=\"register\""));
        assert!(register.text().contains("autocomplete=\"new-password\""));
        request.get("/static/account.js").await.assert_status_ok();
    })
    .await;
}
