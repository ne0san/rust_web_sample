use infra::entity::m_ng_word;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // created_user_idカラムの型をstringからintegerに変更
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(m_ng_word::Entity)
                    .modify_column(
                        ColumnDef::new(m_ng_word::Column::CreatedUserId)
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 元のstring型に戻す
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(m_ng_word::Entity)
                    .modify_column(
                        ColumnDef::new(m_ng_word::Column::CreatedUserId)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
