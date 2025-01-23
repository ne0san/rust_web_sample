use async_trait::async_trait;
use domain_model::register_ng_word::{err::ServiceError, model::NgWord};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RegisterNgWordRepository: Send + Sync {
    /// NGワードを登録する
    async fn register_ng_word(&self, ng_word: &NgWord, user_id: i32) -> Result<(), ServiceError>;
}
