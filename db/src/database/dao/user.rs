use crate::helpers::ShaHelper;
use crate::sea_orm::DatabaseConnection;
use crate::Kind;
use crate::{database::UserRepo, Result};

use abi::{
    pb::types::{GetUserInfoParams, UserBase, UserLogin, UserRegister, UserUnregister, UserUpdate},
    tonic::async_trait,
};
use entity::sea_orm::TransactionTrait;
use entity::{
    sea_orm::{entity::*, ActiveModelTrait, EntityTrait, IntoActiveModel, QueryFilter, Set},
    user::{
        UserAuthActiveModel, UserAuthColumn, UserAuthEntity, UserAuthModel, UserBaseActiveModel,
        UserBaseEntity, UserBaseModel,
    },
};

#[derive(Debug)]
pub struct DaoUser {
    connection: DatabaseConnection,
}

impl DaoUser {
    pub fn new(connection: DatabaseConnection) -> Self {
        DaoUser { connection }
    }

    async fn find_user_by_id(&self, id: i32) -> Result<Option<UserBaseModel>> {
        let model = UserBaseEntity::find_by_id(id).one(&self.connection).await?;

        Ok(model)
    }

    async fn find_user_auth(
        &self,
        auth_type: i32,
        auth_name: &str,
    ) -> Result<Option<UserAuthModel>> {
        let model = UserAuthEntity::find()
            .filter(UserAuthColumn::AuthType.eq(auth_type))
            .filter(UserAuthColumn::AuthName.eq(auth_name))
            .one(&self.connection)
            .await?;

        Ok(model)
    }
}

#[async_trait]
impl UserRepo for DaoUser {
    async fn register(&self, register: UserRegister, sha_helper: &ShaHelper) -> Result<UserBase> {
        let beigin = self.connection.begin().await?;

        let user_model: UserBaseActiveModel = register.clone().into_active_model();

        let user_model = user_model.insert(&beigin).await?;

        let encode_data = sha_helper.encode(register.auth_code.as_bytes());
        let mut auth_model: UserAuthActiveModel = register.into_active_model();
        auth_model.auth_code = Set(encode_data);
        auth_model.user_id = Set(user_model.id);

        let _auth_model = auth_model.insert(&beigin).await?;

        beigin.commit().await?;

        Ok(UserBase::from(user_model))
    }
    async fn unregister(&self, unregister: UserUnregister) -> Result<UserBase> {
        let user_model: UserBaseActiveModel = unregister.into_active_model();

        let user_model = user_model.update(&self.connection).await?;

        Ok(UserBase::from(user_model))
    }
    async fn login(&self, login: UserLogin, sha_helper: &ShaHelper) -> Result<UserBase> {
        let auth_model = self
            .find_user_auth(login.auth_type, &login.auth_name)
            .await?
            .ok_or(Kind::AuthNotFound)?;

        let encode_data = sha_helper.encode(login.auth_code.as_bytes());

        if auth_model.auth_code != encode_data {
            return Err(Kind::PasswordError.into());
        }

        let user_model = self
            .find_user_by_id(auth_model.user_id)
            .await?
            .ok_or(Kind::UserNotFound)?;

        Ok(UserBase::from(user_model))
    }
    async fn get_user_info(&self, param: GetUserInfoParams) -> Result<UserBase> {
        let user_model = self
            .find_user_by_id(param.id)
            .await?
            .ok_or(Kind::UserNotFound)?;

        Ok(UserBase::from(user_model))
    }
    async fn update(&self, _update: UserUpdate) -> Result<UserBase> {
        todo!()
    }
}
