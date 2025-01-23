use async_trait::async_trait;
use domain_model::register_ng_word::{err::ServiceError, model::NgWord};
use domain_service::register_ng_word::repository::RegisterNgWordRepository;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::entity::m_ng_word;

pub struct RegisterNgWordRepositoryImpl {
    conn: DatabaseConnection,
}

impl RegisterNgWordRepositoryImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl RegisterNgWordRepository for RegisterNgWordRepositoryImpl {
    async fn register_ng_word(&self, ng_word: &NgWord, user_id: i32) -> Result<(), ServiceError> {
        let model = m_ng_word::ActiveModel {
            word: Set(ng_word.value().to_string()),
            created_user_id: Set(Some(user_id)),
            ..Default::default()
        };

        model
            .insert(&self.conn)
            .await
            .map_err(|e| ServiceError(e.to_string()))?;

        Ok(())
    }
}
