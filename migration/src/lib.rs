pub use sea_orm_migration::prelude::*;

mod m20240610_061903_create_user_base;
mod m20240610_062346_create_user_auth;
mod m20240612_085658_create_label_meta;
mod m20240612_085711_create_user_label;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240610_061903_create_user_base::Migration),
            Box::new(m20240610_062346_create_user_auth::Migration),
            Box::new(m20240612_085658_create_label_meta::Migration),
            Box::new(m20240612_085711_create_user_label::Migration),
        ]
    }
}
