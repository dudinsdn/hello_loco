use hello_loco::{app::App, models::posts};
use loco_rs::{task, testing::prelude::*};
use sea_orm::{ActiveModelTrait, Set};

use loco_rs::boot::run_task;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_can_run_posts_report() {
    let boot = boot_test::<App>().await.unwrap();

    posts::ActiveModel {
        title: Set(Some("Tour post".to_string())),
        content: Set(None),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();
    posts::ActiveModel {
        title: Set(None),
        content: Set(None),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();
    let report = posts::Model::report(&boot.app_context.db).await.unwrap();
    assert_eq!(report.len(), 2);
    assert_eq!(report[0].title.as_deref(), Some("Tour post"));
    assert_eq!(report[1].title, None);

    assert!(run_task::<App>(
        &boot.app_context,
        Some(&"posts_report".to_string()),
        &task::Vars::default()
    )
    .await
    .is_ok());
}
