pub use sea_orm_migration::prelude::*;

mod m20250214_030446_create_label;
mod m20250214_030518_create_user;
mod m20250214_030537_create_user_base;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250214_030446_create_label::Migration),
            Box::new(m20250214_030518_create_user::Migration),
            Box::new(m20250214_030537_create_user_base::Migration),
        ]
    }
}
