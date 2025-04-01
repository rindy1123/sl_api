//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sex")]
pub enum Sex {
    #[sea_orm(string_value = "male")]
    Male,
    #[sea_orm(string_value = "female")]
    Female,
}
