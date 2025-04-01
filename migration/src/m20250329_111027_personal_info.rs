use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Sex)
                    .values(SexVariants::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PersonalInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(PersonalInfo::Id))
                    .col(string(PersonalInfo::Occupation))
                    .col(integer(PersonalInfo::Age))
                    .col(enumeration(PersonalInfo::Sex, Sex, SexVariants::iter()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PersonalInfo::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().if_exists().name(Sex).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PersonalInfo {
    Table,
    Id,
    Occupation,
    Sex,
    Age,
}

#[derive(DeriveIden)]
struct Sex;

#[derive(DeriveIden, EnumIter)]
pub enum SexVariants {
    Male,
    Female,
}
