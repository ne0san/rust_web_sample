use crate::entity::{
    m_ng_word::{self, Column},
    m_user_name,
};
use async_trait::async_trait;
use domain_model::register_user_name::{
    err::{RegisterUserNameError, ServiceError},
    model::UserName,
};
use domain_service::register_user_name::RegisterUserNameRepository;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tracing::error;

pub struct RegisterUserNameRepositoryImpl {
    db_conn: Arc<DatabaseConnection>,
}
#[async_trait]
impl RegisterUserNameRepository for RegisterUserNameRepositoryImpl {
    async fn find_ng_word(&self, user_name: &UserName) -> Result<bool, RegisterUserNameError> {
        let pattern = format!("%{}%", user_name.value());
        let result = m_ng_word::Entity::find()
            .filter(Expr::col(Column::Word).like(&pattern))
            .count(self.db_conn.as_ref())
            .await;
        match result {
            Ok(count) => Ok(count > 0),
            Err(err) => {
                error!("Failed to find ng word: {:?}", err);
                Err(RegisterUserNameError::from(ServiceError(
                    "Failed to find ng word".to_string(),
                )))
            }
        }
    }
    async fn create_user_name(&self, user_name: &UserName) -> Result<(), ServiceError> {
        unimplemented!()
    }
}
