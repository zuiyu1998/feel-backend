pub use sea_orm_migration::prelude::*;

mod m20250113_082151_create_user;
mod m20250113_095101_create_user_auth;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250113_082151_create_user::Migration),
            Box::new(m20250113_095101_create_user_auth::Migration),
        ]
    }
}
