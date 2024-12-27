pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_ng_word;
mod m20241227_123201_create_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_ng_word::Migration),
            Box::new(m20241227_123201_create_index::Migration),
        ]
    }
}
