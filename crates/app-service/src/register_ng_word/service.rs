use async_trait::async_trait;
use domain_model::register_ng_word::err::RegisterNgWordError;
use domain_service::register_ng_word::DomainService as RegisterNgWordDomainService;
use std::sync::Arc;
use tracing::{error, info};

#[async_trait]
pub trait AppService: Send + Sync {
    async fn register_ng_word(
        &self,
        ng_word: &str,
        user_id: i32,
    ) -> Result<(), RegisterNgWordError>;
}

pub struct AppServiceImpl {
    register_ng_word_domain_service: Arc<dyn RegisterNgWordDomainService>,
}

impl AppServiceImpl {
    pub fn new(register_ng_word_domain_service: Arc<dyn RegisterNgWordDomainService>) -> Self {
        AppServiceImpl {
            register_ng_word_domain_service,
        }
    }
}

#[async_trait]
impl AppService for AppServiceImpl {
    async fn register_ng_word(
        &self,
        ng_word: &str,
        user_id: i32,
    ) -> Result<(), RegisterNgWordError> {
        let result = self
            .register_ng_word_domain_service
            .register_ng_word(ng_word, user_id)
            .await;

        if let Err(err) = &result {
            error!("Failed to register ng word: {:?}", err);
        } else {
            info!("Successfully registered ng word: {}", ng_word);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {
        use super::*;
        use domain_model::register_ng_word::err::{ServiceError, ValidationError};
        use mockall::{mock, predicate::*};

        mock! {
            pub DomainService {}
            #[async_trait]
            impl RegisterNgWordDomainService for DomainService {
                async fn register_ng_word(&self, ng_word: &str, user_id: i32) -> Result<(), RegisterNgWordError>;
            }
        }

        #[tokio::test]
        async fn test_register_ng_word() {
            let mut mock = MockDomainService::new();
            mock.expect_register_ng_word()
                .with(eq("test"), eq(1))
                .times(1)
                .returning(|_, _| Ok(()));

            let app_service = AppServiceImpl::new(Arc::new(mock));
            let result = app_service.register_ng_word("test", 1).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_register_ng_word_validation_error() {
            let mut mock = MockDomainService::new();
            mock.expect_register_ng_word()
                .with(eq("te"), eq(1))
                .times(1)
                .returning(|_, _| {
                    Err(RegisterNgWordError::ValidationError(ValidationError(
                        "validation error".to_string(),
                    )))
                });

            let app_service = AppServiceImpl::new(Arc::new(mock));
            let result = app_service.register_ng_word("te", 1).await;
            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(RegisterNgWordError::ValidationError(_))
            ));
        }

        #[tokio::test]
        async fn test_register_ng_word_service_error() {
            let mut mock = MockDomainService::new();
            mock.expect_register_ng_word()
                .with(eq("test"), eq(1))
                .times(1)
                .returning(|_, _| {
                    Err(RegisterNgWordError::ServiceError(ServiceError(
                        "service error".to_string(),
                    )))
                });

            let app_service = AppServiceImpl::new(Arc::new(mock));
            let result = app_service.register_ng_word("test", 1).await;
            assert!(result.is_err());
            assert!(matches!(result, Err(RegisterNgWordError::ServiceError(_))));
        }
    }
}
