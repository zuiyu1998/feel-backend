pub use sea_orm_migration::prelude::*;

mod m20250214_030446_create_label;
mod m20250218_025320_create_user_base;
mod m20250218_025328_create_user_auth;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250214_030446_create_label::Migration),
            Box::new(m20250218_025320_create_user_base::Migration),
            Box::new(m20250218_025328_create_user_auth::Migration),
        ]
    }
}
