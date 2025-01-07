use async_trait::async_trait;
use domain_model::get_all_post::{err::ServiceError, model::Post};
use domain_service::get_all_post::DomainService as GetAllPostDomainService;
use std::sync::Arc;
use tracing::{error, info};

#[async_trait]
pub trait AppService: Send + Sync {
    async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError>;
}

pub struct AppServiceImpl {
    get_all_post_domain_service: Arc<dyn GetAllPostDomainService>,
}
impl AppServiceImpl {
    pub fn new(get_all_post_domain_service: Arc<dyn GetAllPostDomainService>) -> Self {
        Self {
            get_all_post_domain_service,
        }
    }
}
#[async_trait]
impl AppService for AppServiceImpl {
    async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError> {
        let result = self.get_all_post_domain_service.get_all_post().await;

        if let Err(err) = &result {
            error!("Failed to get all post: {:?}", err);
        } else {
            info!("Successfully get all post");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {
        use super::*;
        use chrono::NaiveDateTime;
        use domain_model::get_all_post::model::{Content, Post, PostedDatetime, PostedUserName};
        use mockall::{mock, predicate::*};
        const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

        mock! {
            pub DomainService{}
            #[async_trait]
            impl GetAllPostDomainService for DomainService{
                async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError>;
            }
        }

        #[tokio::test]
        async fn test_get_all_post() {
            let mut domain_service = MockDomainService::new();
            domain_service.expect_get_all_post().times(1).returning(|| {
                Ok(vec![Post {
                    posted_user_name: PostedUserName("test".to_string()),
                    posted_datetime: PostedDatetime(
                        NaiveDateTime::parse_from_str("2021-01-01T00:00:00Z", DATETIME_FORMAT)
                            .unwrap(),
                    ),
                    content: Content("test".to_string()),
                }])
            });

            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let result = app_service.get_all_post().await;

            assert!(result.is_ok());
        }
    }
}
