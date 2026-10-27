use loco_rs::prelude::*;

pub struct PostsReport;
#[async_trait]
impl Task for PostsReport {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "posts_report".to_string(),
            detail: "Print every post’s title".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let posts = crate::models::posts::Entity::find()
            .all(&app_context.db)
            .await?;
        for post in &posts {
            println!("- {}", post.title.as_deref().unwrap_or("(untitled)"));
        }
        println!("done: {} posts", posts.len());
        Ok(())
    }
}
