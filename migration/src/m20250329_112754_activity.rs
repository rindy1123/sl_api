use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250329_111027_day::Day;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Activity::Table)
                    .if_not_exists()
                    .col(pk_auto(Activity::Id))
                    .col(integer(Activity::Hours))
                    .col(string(Activity::Name))
                    .col(string(Activity::Color))
                    .col(integer(Activity::DayId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activity-day")
                            .from(Activity::Table, Activity::DayId)
                            .to(Day::Table, Day::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Activity::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Activity {
    Table,
    Id,
    DayId,
    Hours,
    Name,
    Color,
}
