#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20261026_121733_posts;
mod m20261027_034242_comments;
mod m20261027_044737_notes;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20261026_121733_posts::Migration),
            Box::new(m20261027_034242_comments::Migration),
            Box::new(m20261027_044737_notes::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
