pub use super::_entities::posts::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Posts = Entity;

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
    pub async fn comments(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<super::comments::Model>, DbErr> {
        use sea_orm::QueryOrder;
        self.find_related(super::comments::Entity)
            .order_by_asc(super::_entities::comments::Column::Id)
            .all(db)
            .await
    }

    pub async fn report(db: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        use sea_orm::QueryOrder;
        Entity::find()
            .order_by_asc(super::_entities::posts::Column::Id)
            .all(db)
            .await
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
