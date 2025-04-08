use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Day::Table)
                    .if_not_exists()
                    .col(pk_auto(Day::Id))
                    .col(string(Day::Title))
                    .col(string_null(Day::Description))
                    .col(string(Day::Occupation))
                    .col(string(Day::Country))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Day::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Day {
    Table,
    Id,
    Title,
    Description,
    Occupation,
    Country,
    CreatedAt,
}
