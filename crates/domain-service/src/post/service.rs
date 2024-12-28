use super::repository::PostRepository;
use async_trait::async_trait;
use domain_model::post::{err::PostError, model::Post};
use std::sync::Arc;

#[async_trait]
pub trait DomainService: Send + Sync {
    async fn post(&self, post: Post) -> Result<(), PostError>;
}

pub struct DomainServiceImpl {
    post_repository: Arc<dyn PostRepository>,
}
impl DomainServiceImpl {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        DomainServiceImpl { post_repository }
    }
}
// #[async_trait]

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mod domain_service_impl {
        use super::*;

        #[tokio::test]
        async fn test_post() {}
    }
}
