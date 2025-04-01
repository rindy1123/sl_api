use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250329_111027_personal_info::PersonalInfo;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schedule::Table)
                    .if_not_exists()
                    .col(pk_auto(Schedule::Id))
                    .col(integer(Schedule::StartTime))
                    .col(string(Schedule::Label))
                    .col(integer(Schedule::Order))
                    .col(integer(Schedule::PersonalInfoId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-schedule-personal_info")
                            .from(Schedule::Table, Schedule::PersonalInfoId)
                            .to(PersonalInfo::Table, PersonalInfo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("uniq-idx-schedule_order_personal_info_id")
                    .table(Schedule::Table)
                    .col(Schedule::Order)
                    .col(Schedule::PersonalInfoId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("uniq-idx-schedule_order")
                    .table(Schedule::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Schedule::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Schedule {
    Table,
    Id,
    PersonalInfoId,
    StartTime,
    Label,
    Order,
}
