use crate::entity::m_user_name;
use async_trait::async_trait;
use domain_model::register_user_name::{
    err::{RegisterUserNameError, ServiceError},
    model::UserName,
};
use domain_service::register_user_name::RegisterUserNameRepository;
use sea_orm::{entity::prelude::*, DatabaseConnection, DbBackend, Set, Statement};
use std::sync::Arc;
use tracing::error;

pub struct RegisterUserNameRepositoryImpl {
    db_conn: Arc<DatabaseConnection>,
}
impl RegisterUserNameRepositoryImpl {
    pub fn new(db_conn: Arc<DatabaseConnection>) -> Self {
        RegisterUserNameRepositoryImpl { db_conn }
    }
}
#[async_trait]
impl RegisterUserNameRepository for RegisterUserNameRepositoryImpl {
    async fn find_ng_word(&self, user_name: &UserName) -> Result<bool, RegisterUserNameError> {
        let query = r#"
            SELECT count(*) cnt
            FROM m_ng_word mnw
            WHERE ? LIKE CONCAT('%', mnw.word, '%');
        "#;

        let statement =
            Statement::from_sql_and_values(DbBackend::MySql, query, vec![user_name.value().into()]);

        let result = self.db_conn.query_one(statement).await;

        match result {
            Ok(Some(row)) => match row.try_get::<i64>("", "cnt") {
                Ok(count) => Ok(count > 0),
                Err(err) => {
                    error!("Failed to find ng word: {:?}", err);
                    Err(RegisterUserNameError::from(ServiceError(
                        "Failed to find ng word".to_string(),
                    )))
                }
            },
            Ok(None) => Ok(false),
            Err(err) => {
                error!("Failed to find ng word: {:?}", err);
                Err(RegisterUserNameError::from(ServiceError(
                    "Failed to find ng word".to_string(),
                )))
            }
        }
    }
    async fn create_user_name(&self, user_name: &UserName) -> Result<(), ServiceError> {
        let user_name = m_user_name::ActiveModel {
            name: Set(user_name.value().to_string()),
            ..Default::default()
        };

        let result = user_name.insert(self.db_conn.as_ref()).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to create user name: {:?}", err);
                Err(ServiceError("Failed to create user name".to_string()))
            }
        }
    }
}
