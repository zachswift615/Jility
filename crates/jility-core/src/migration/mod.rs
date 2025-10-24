pub use sea_orm_migration::prelude::*;

mod m20241024_000001_create_initial_schema;
mod m20241024_000002_add_fts;
mod m20241024_000003_add_auth_tables;
mod m20241024_000004_extend_projects;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241024_000001_create_initial_schema::Migration),
            Box::new(m20241024_000002_add_fts::Migration),
            Box::new(m20241024_000003_add_auth_tables::Migration),
            Box::new(m20241024_000004_extend_projects::Migration),
        ]
    }
}
