mod user_auth;
mod user_base;

pub use user_base::{
    ActiveModel as UserBaseActiveModel, Column as UserBaseColumn, Entity as UserBaseEntity,
};

pub use user_auth::{
    ActiveModel as UserAuthActiveModel, Column as UserAuthColumn, Entity as UserAuthEntity,
};
