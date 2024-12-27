use async_trait::async_trait;
use domain_model::register_user_name::{
    err::{RegisterUserNameError, ServiceError},
    model::UserName,
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RegisterUserNameRepository: Send + Sync {
    async fn find_ng_word(&self, user_name: &UserName) -> Result<bool, RegisterUserNameError>;
    // SELECT count(*) FROM ng_word WHERE ? LIKE CONCAT('%', word, '%'); みたいな感じで実装する
    async fn create_user_name(&self, user_name: &UserName) -> Result<(), ServiceError>;
}
