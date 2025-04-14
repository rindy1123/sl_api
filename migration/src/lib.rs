pub use sea_orm_migration::prelude::*;

mod m20250329_111027_day;
mod m20250329_112754_activity;
mod m20250408_102850_add_created_at_to_day;
mod m20250414_111155_alter_hours_to_float;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250329_111027_day::Migration),
            Box::new(m20250329_112754_activity::Migration),
            Box::new(m20250408_102850_add_created_at_to_day::Migration),
            Box::new(m20250414_111155_alter_hours_to_float::Migration),
        ]
    }
}
