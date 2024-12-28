use async_trait::async_trait;
use domain_model::post::{
    err::ServiceError,
    model::{NgWord, Post},
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_all_ng_word(&self) -> Result<Vec<NgWord>, ServiceError>;
    async fn create_post(&self, post: &Post) -> Result<(), ServiceError>;
}
