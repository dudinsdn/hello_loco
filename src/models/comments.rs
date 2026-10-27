pub use super::_entities::comments::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Comments = Entity;

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
    pub async fn create_for_post(
        db: &DatabaseConnection,
        post_id: i64,
        content: Option<String>,
    ) -> loco_rs::model::ModelResult<Self> {
        use sea_orm::{ActiveValue::Set, TransactionTrait};
        let txn = db.begin().await?;
        super::posts::Entity::find_by_id(post_id)
            .one(&txn)
            .await?
            .ok_or(loco_rs::model::ModelError::EntityNotFound)?;
        let comment = ActiveModel {
            content: Set(content),
            post_id: Set(post_id),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        Ok(comment)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
