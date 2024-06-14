use crate::{database::LabelRepo, Result, DATABASE_BACKEND};

use abi::pb::types::{
    UserLabel, UserLabelCreate, UserLabelCreateResponse, UserLabelMetaCreate,
    UserLabelMetaCreateResponse, UserLabelParams, UserLabelResponse,
};
use abi::sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait,
    QuerySelect, Statement,
};
use abi::tonic::async_trait;
use entity::label::{LabelMetaActiveModel, UserLabelActiveModel, UserLabelEntity};

#[derive(Debug)]
pub struct DaoLabel {
    connection: DatabaseConnection,
}

impl DaoLabel {
    pub fn new(connection: DatabaseConnection) -> Self {
        DaoLabel { connection }
    }
}

#[async_trait]
impl LabelRepo for DaoLabel {
    async fn get_user_labels(&self, params: UserLabelParams) -> Result<UserLabelResponse> {
        let sql = r#"select ul.id as id, lm."name" as name, lm.description as description, lm.effct as effct, ul.create_at as create_at, ul.update_at as update_at from user_label ul
        join label_meta lm on lm.id = ul.label_meta_id
        where ul.user_id  = $1"#;

        let stmt = Statement::from_sql_and_values(DATABASE_BACKEND, sql, [params.id.into()]);

        let count = UserLabelEntity::find()
            .select_only()
            .from_raw_sql(stmt.clone())
            .count(&self.connection)
            .await? as i32;

        let paginator = UserLabelEntity::find()
            .select_only()
            .from_raw_sql(stmt)
            .into_model::<UserLabel>()
            .paginate(&self.connection, params.page_size as u64);

        let labels = paginator.fetch_page(params.page as u64).await?;

        let mut has_next = true;

        if labels.len() < params.page_size as usize {
            has_next = false;
        }

        Ok(UserLabelResponse {
            count,
            page: params.page,
            page_size: params.page_size,
            has_next,
            data: labels,
        })
    }

    async fn create_user_lable(&self, create: UserLabelCreate) -> Result<UserLabelCreateResponse> {
        let model: UserLabelActiveModel = create.into_active_model();

        let model = model.insert(&self.connection).await?;

        Ok(UserLabelCreateResponse { id: model.id })
    }
    async fn create_lable_meta(
        &self,
        create: UserLabelMetaCreate,
    ) -> Result<UserLabelMetaCreateResponse> {
        let model: LabelMetaActiveModel = create.into_active_model();

        let model = model.insert(&self.connection).await?;

        Ok(UserLabelMetaCreateResponse { id: model.id })
    }
}
