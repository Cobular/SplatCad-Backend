pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_start;
mod m20220810_230432_seed;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_start::Migration),
            Box::new(m20220810_230432_seed::Migration),
        ]
    }
}
