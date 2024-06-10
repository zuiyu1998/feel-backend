mod dao;

mod user;

use crate::{sea_orm::Database, Error};
use abi::{
    config::{Config, FromConfig},
    tonic::async_trait,
};
use dao::*;
use migration::{Migrator, MigratorTrait};
use user::*;

pub struct DbRepo {
    pub user: Box<dyn UserRepo>,
}

#[async_trait]
impl FromConfig for DbRepo {
    type Error = Error;

    async fn from_config(config: &Config) -> Result<Self, Self::Error> {
        let connction = Database::connect(&config.db.postgres).await?;

        Migrator::up(&connction, None).await?;

        Ok(DbRepo {
            user: Box::new(DaoUser::new(connction.clone())),
        })
    }
}
