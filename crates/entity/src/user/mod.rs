pub mod user_base;
pub mod user_auth;

pub use user_base::{Column as UserBaseColumn, Entity as UserBaseEntity};
pub use user_auth::{Column as UserAuthColumn, Entity as UserAuthEntity};
