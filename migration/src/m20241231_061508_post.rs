use infra::entity::t_post;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(t_post::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(t_post::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(t_post::Column::PostedUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(t_post::Column::Content).string().not_null())
                    .col(
                        ColumnDef::new(t_post::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(t_post::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("fk_posted_user_id")
                    .from(t_post::Entity, t_post::Column::PostedUserId)
                    .to(t_post::Entity, t_post::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(t_post::Entity).to_owned())
            .await
    }
}
