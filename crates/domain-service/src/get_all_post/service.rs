use super::repository::GetAllPostRepository;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use domain_model::get_all_post::{
    err::ServiceError,
    model::{Content, Post, PostedDatetime, PostedUserName},
};
use std::sync::Arc;

#[async_trait]
pub trait DomainService: Send + Sync {
    async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError>;
}

pub struct DomainServiceImpl {
    get_all_post_repository: Arc<dyn GetAllPostRepository>,
}
impl DomainServiceImpl {
    pub fn new(get_all_post_repository: Arc<dyn GetAllPostRepository>) -> Self {
        Self {
            get_all_post_repository,
        }
    }
}

#[async_trait]
impl DomainService for DomainServiceImpl {
    async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError> {
        self.get_all_post_repository.get_all_post().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mod domain_service_impl {
        use super::*;
        use crate::get_all_post::repository::MockGetAllPostRepository;

        #[tokio::test]
        async fn test_get_all_post() {
            let mut mock = MockGetAllPostRepository::new();
            mock.expect_get_all_post().times(1).returning(|| {
                Ok(vec![Post {
                    posted_user_name: PostedUserName("test".to_string()),
                    posted_datetime: PostedDatetime("2021-01-01T00:00:00Z"),
                    content: Content("test".to_string()),
                }])
            });

            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.get_all_post().await;

            assert!(result.is_ok());
            assert_eq!(
                result,
                Ok(vec![Post {
                    posted_user_name: PostedUserName("test".to_string()),
                    posted_datetime: PostedDatetime("2021-01-01T00:00:00Z".to_string()),
                    content: Content("test".to_string()),
                }])
            );
        }
    }
}
