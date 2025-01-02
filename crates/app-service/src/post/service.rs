use async_trait::async_trait;
use domain_model::post::{err::PostError, model::UnvalidatedPost};
use domain_service::post::DomainService as PostDomainService;
use std::sync::Arc;
use tracing::{error, info};

#[async_trait]
pub trait AppService: Send + Sync {
    async fn post(&self, post: UnvalidatedPost) -> Result<(), PostError>;
}

pub struct AppServiceImpl {
    post_domain_service: Arc<dyn PostDomainService>,
}
impl AppServiceImpl {
    pub fn new(post_domain_service: Arc<dyn PostDomainService>) -> Self {
        AppServiceImpl {
            post_domain_service,
        }
    }
}
#[async_trait]
impl AppService for AppServiceImpl {
    async fn post(&self, post: UnvalidatedPost) -> Result<(), PostError> {
        let result = self.post_domain_service.post(post.clone()).await;

        if let Err(err) = &result {
            error!("Failed to post: {:?} ", err);
        } else {
            info!("Successfully posted: {:?}", &post);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {
        use super::*;
        use domain_model::post::{
            err::{ServiceError, ValidationError},
            model::{UnvalidatedPost, UnvalidatedUserName},
        };
        use mockall::{mock, predicate::*};

        mock! {
            pub DomainService {}
            #[async_trait]
            impl PostDomainService for DomainService {
                async fn post(&self, post: UnvalidatedPost) -> Result<(), PostError>;
            }
        }

        #[tokio::test]
        async fn post() {
            let mut mock = MockDomainService::new();
            let post = UnvalidatedPost {
                user_name: UnvalidatedUserName("user_name".to_string()),
                content: "content".to_string(),
            };
            mock.expect_post()
                .with(eq(post.clone()))
                .times(1)
                .returning(|_| Ok(()));

            let app_service = AppServiceImpl::new(Arc::new(mock));
            let result = app_service.post(post).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn post_error() {
            let mut mock = MockDomainService::new();
            let post = UnvalidatedPost {
                user_name: UnvalidatedUserName("user_name".to_string()),
                content: "content".to_string(),
            };
            mock.expect_post()
                .with(eq(post.clone()))
                .times(1)
                .returning(|_| Err(PostError::from(ValidationError("error".to_string()))));

            let app_service = AppServiceImpl::new(Arc::new(mock));
            let result = app_service.post(post).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn post_service_error() {
            let mut mock = MockDomainService::new();
            let post = UnvalidatedPost {
                user_name: UnvalidatedUserName("user_name".to_string()),
                content: "content".to_string(),
            };
            mock.expect_post()
                .with(eq(post.clone()))
                .times(1)
                .returning(|_| Err(PostError::from(ServiceError("error".to_string()))));

            let app_service = AppServiceImpl::new(Arc::new(mock));
            let result = app_service.post(post).await;
            assert!(result.is_err());
        }
    }
}
