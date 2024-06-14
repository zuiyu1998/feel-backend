use abi::pb::types::{UserLabelCreate, UserLabelMetaCreate, UserLabelParams};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Object, Validate)]
pub struct UserLabelParamsRequest {
    pub page: i32,
    pub page_size: i32,
    pub id: i32,
}

impl UserLabelParamsRequest {
    pub fn into_inner(self) -> UserLabelParams {
        UserLabelParams {
            page: self.page,
            page_size: self.page_size,
            id: self.id,
        }
    }
}

#[derive(Deserialize, Serialize, Object, Validate)]
pub struct UserLabelCreateRequest {
    pub user_id: i32,
    pub label_meta_id: i32,
}

impl UserLabelCreateRequest {
    pub fn into_inner(self) -> UserLabelCreate {
        UserLabelCreate {
            label_meta_id: self.label_meta_id,
            user_id: self.user_id,
        }
    }
}

#[derive(Deserialize, Serialize, Object, Validate)]
pub struct UserLabelMetaCreateRequest {
    #[validate(length(min = 2, max = 16))]
    pub name: String,
    #[validate(length(min = 2, max = 200))]
    pub description: String,
    pub effct: i64,
}

impl UserLabelMetaCreateRequest {
    pub fn into_inner(self) -> UserLabelMetaCreate {
        UserLabelMetaCreate {
            name: self.name,
            description: self.description,
            effct: self.effct,
        }
    }
}
