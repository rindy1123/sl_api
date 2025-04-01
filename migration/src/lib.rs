pub use sea_orm_migration::prelude::*;

mod m20250329_111027_personal_info;
mod m20250329_112754_schedule;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250329_111027_personal_info::Migration),
            Box::new(m20250329_112754_schedule::Migration),
        ]
    }
}
