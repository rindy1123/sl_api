use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250329_112754_activity::Activity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Activity::Table)
                    .modify_column(float(Activity::Hours))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Activity::Table)
                    .modify_column(integer(Activity::Hours))
                    .to_owned(),
            )
            .await
    }
}
