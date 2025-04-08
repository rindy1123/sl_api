use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250329_111027_day::Day;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Day::Table)
                    .add_column_if_not_exists(
                        timestamp_with_time_zone(Day::CreatedAt).default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Day::Table)
                    .drop_column(Day::CreatedAt)
                    .to_owned(),
            )
            .await
    }
}
