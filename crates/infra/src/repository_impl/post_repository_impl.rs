use crate::entity::{m_ng_word, m_user_name, t_post};
use async_trait::async_trait;
use domain_model::post::{
    err::ServiceError,
    model::{NgWord, Post, UserName},
};
use domain_service::post::PostRepository;
use sea_orm::{
    entity::prelude::*, ActiveValue::NotSet, DatabaseConnection, EntityOrSelect, QuerySelect, Set,
};
use tracing::error;
pub struct PostRepositoryImpl {
    db_conn: DatabaseConnection,
}
impl PostRepositoryImpl {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn user_exists(&self, user: &UserName) -> Result<bool, ServiceError> {
        let count = m_user_name::Entity::find()
            .filter(m_user_name::Column::Name.contains(user.value()))
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
    async fn find_all_ng_word(&self) -> Result<Vec<NgWord>, ServiceError> {
        let ng_words = m_ng_word::Entity::find().all(&self.db_conn).await;
        match ng_words {
            Ok(ng_words) => Ok(ng_words.into_iter().map(|ng_word| ng_word.into()).collect()),
            Err(err) => {
                error!("Failed to find ng words: {:?}", err);
                Err(ServiceError("Failed to find ng words".to_string()))
            }
        }
    }
    async fn create_post(&self, post: &Post) -> Result<(), ServiceError> {
        let user = m_user_name::Entity::find()
            .filter(m_user_name::Column::Name.eq(post.user_name().value()))
            .select()
            .column(m_user_name::Column::Id)
            .one(&self.db_conn)
            .await;
        let post = t_post::ActiveModel {
            id: NotSet,
            posted_user_id: Set(user.unwrap().unwrap().id),
            content: Set(post.content().to_string()),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let result = t_post::Entity::insert(post).exec(&self.db_conn).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to create post: {:?}", err);
                Err(ServiceError("Failed to create post".to_string()))
            }
        }
    }
}
