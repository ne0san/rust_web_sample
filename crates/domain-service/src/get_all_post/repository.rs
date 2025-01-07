use async_trait::async_trait;
use domain_model::get_all_post::{err::ServiceError, model::Post};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GetAllPostRepository: Send + Sync {
    async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError>;
}
