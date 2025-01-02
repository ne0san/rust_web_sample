use crate::entity::m_user_name;
use async_trait::async_trait;
use domain_model::login::{err::ServiceError, model::UncheckedUserName};
use domain_service::login::LoginRepository;
use sea_orm::{entity::prelude::*, DatabaseConnection};
use tracing::error;

pub struct LoginRepositoryImpl {
    db_conn: DatabaseConnection,
}
impl LoginRepositoryImpl {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

#[async_trait]
impl LoginRepository for LoginRepositoryImpl {
    async fn user_exists(&self, user: &UncheckedUserName) -> Result<bool, ServiceError> {
        let count = m_user_name::Entity::find()
            .filter(m_user_name::Column::Name.contains(user.0.clone()))
            .count(&self.db_conn)
            .await;
        match count {
            Ok(count) => Ok(count > 0),
            Err(err) => {
                error!("Failed to find user: {:?}", err);
                Err(ServiceError("Failed to find user".to_string()))
            }
        }
    }
}
