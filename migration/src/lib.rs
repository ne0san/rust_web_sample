pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_ng_word;
mod m20241231_061508_post;
mod m20250123_074710_modify_ng_word_created_user_id_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_ng_word::Migration),
            Box::new(m20241231_061508_post::Migration),
            Box::new(m20250123_074710_modify_ng_word_created_user_id_type::Migration),
        ]
    }
}
