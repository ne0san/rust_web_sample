use async_trait::async_trait;
use domain_model::login::{err::ServiceError, model::UncheckedUserName};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait LoginRepository: Send + Sync {
    async fn user_exists(&self, user: &UncheckedUserName) -> Result<bool, ServiceError>;
}
