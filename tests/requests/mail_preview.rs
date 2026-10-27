use hello_loco::{app::App, mailers::auth::AuthMailer};
use loco_rs::{app::Hooks, environment::Environment, testing::prelude::*};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn previews_render_real_templates_without_sending_mail() {
    request::<App, _, _>(|request, mut ctx| async move {
        for kind in ["welcome", "forgot", "magic_link"] {
            let response = request.get(&format!("/mail-preview/{kind}")).await;
            response.assert_status_ok();
            assert!(response.text().contains("sandbox=\"\""));
            let preview = AuthMailer::preview(kind).unwrap().unwrap();
            assert!(preview.html.contains("https://example.invalid"));
            assert!(!preview.html.contains("{{"));
            assert!(!preview.text.contains("{{"));
            assert!(!preview.subject.contains("{{"));
        }
        let welcome = AuthMailer::preview("welcome").unwrap().unwrap();
        assert!(welcome.subject_source.contains("{{name}}"));
        assert!(welcome.subject.contains("Rani"));
        assert!(welcome.html.contains("contoh-verifikasi"));
        request
            .get("/mail-preview/unknown")
            .await
            .assert_status_not_found();
        assert_eq!(ctx.mailer.as_ref().unwrap().deliveries().count, 0);
        ctx.environment = Environment::Production;
        let routes = App::routes(&ctx).collect();
        assert!(!format!("{routes:?}").contains("mail-preview"));
    })
    .await;
}
