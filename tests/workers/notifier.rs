use hello_loco::{
    app::App,
    workers::notifier::{Worker, WorkerArgs},
};
use loco_rs::{bgworker::BackgroundWorker, testing::prelude::*};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_notifier_worker() {
    let boot = boot_test::<App>().await.unwrap();

    // Execute the worker ensuring that it operates in 'ForegroundBlocking' mode, which prevents the addition of your worker to the background
    assert!(
        Worker::perform_later(&boot.app_context, WorkerArgs { post_id: 1 })
            .await
            .is_ok()
    );
    // Include additional assert validations after the execution of the worker
}
