pub use super::_entities::notes::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Notes = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Self, DbErr> {
        ActiveModel {
            title: sea_orm::ActiveValue::Set(title),
            content: sea_orm::ActiveValue::Set(content),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
