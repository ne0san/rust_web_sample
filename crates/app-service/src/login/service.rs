use async_trait::async_trait;
use domain_model::login::{err::LoginError, model::UncheckedUserName};
use domain_service::login::DomainService as loginDomainService;
use std::sync::Arc;
use tracing::{error, info};

#[async_trait]
pub trait AppService: Send + Sync {
    async fn login(&self, user_name: UncheckedUserName) -> Result<(), LoginError>;
}

pub struct AppServiceImpl {
    login_domain_service: Arc<dyn loginDomainService>,
}
impl AppServiceImpl {
    pub fn new(login_domain_service: Arc<dyn loginDomainService>) -> Self {
        Self {
            login_domain_service,
        }
    }
}
#[async_trait]
impl AppService for AppServiceImpl {
    async fn login(&self, user_name: UncheckedUserName) -> Result<(), LoginError> {
        let result = self.login_domain_service.login(user_name.clone()).await;

        if let Err(err) = &result {
            error!("Failed to login: {:?} name: {:?}", err, &user_name.0);
        } else {
            info!("Successfully login: {:?}", &user_name.0);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {
        use super::*;
        use domain_model::login::err::{ServiceError, ValidationError};
        use mockall::{mock, predicate::*};

        mock! {
            pub DomainService{}
            #[async_trait]
            impl loginDomainService for DomainService{
                async fn login(&self, user_name: UncheckedUserName) -> Result<(), LoginError>;
            }
        }

        #[tokio::test]
        async fn test_login() {
            let mut domain_service = MockDomainService::new();
            domain_service.expect_login().times(1).returning(|_| Ok(()));

            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let user_name = UncheckedUserName("user_name".to_string());
            let result = app_service.login(user_name).await;

            assert!(result.is_ok());
            assert_eq!(result, Ok(()));
        }

        #[tokio::test]
        async fn test_login_error() {
            let mut domain_service = MockDomainService::new();
            domain_service.expect_login().times(1).returning(|_| {
                Err(LoginError::ValidationError(ValidationError(
                    "error".to_string(),
                )))
            });

            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let user_name = UncheckedUserName("user_name".to_string());
            let result = app_service.login(user_name).await;

            assert!(result.is_err());
            assert_eq!(
                result,
                Err(LoginError::ValidationError(ValidationError(
                    "error".to_string()
                )))
            );
        }

        #[tokio::test]
        async fn test_login_service_error() {
            let mut domain_service = MockDomainService::new();
            domain_service
                .expect_login()
                .times(1)
                .returning(|_| Err(LoginError::ServiceError(ServiceError("error".to_string()))));

            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let user_name = UncheckedUserName("user_name".to_string());
            let result = app_service.login(user_name).await;

            assert!(result.is_err());
            assert_eq!(
                result,
                Err(LoginError::ServiceError(ServiceError("error".to_string())))
            );
        }
    }
}
