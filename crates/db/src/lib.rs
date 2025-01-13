use std::sync::Arc;

use user::UserRepo;

pub mod user;

pub mod database;


pub struct Db {
    user: Arc<dyn UserRepo>
}