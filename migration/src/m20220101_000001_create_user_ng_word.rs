use infra::entity::{m_ng_word, m_user_name};
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(m_user_name::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(m_user_name::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(m_user_name::Column::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(m_user_name::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(m_user_name::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                sea_query::Table::create()
                    .table(m_ng_word::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(m_ng_word::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(m_ng_word::Column::Word)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(m_ng_word::Column::CreatedUserId).string())
                    .col(
                        ColumnDef::new(m_ng_word::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(m_ng_word::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(m_user_name::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(sea_query::Table::drop().table(m_ng_word::Entity).to_owned())
            .await
    }
}
