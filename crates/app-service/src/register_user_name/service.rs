use async_trait::async_trait;
use domain_model::register_user_name::{err::RegisterUserNameError, model::UnvalidatedUserName};
use domain_service::register_user_name::DomainService as RegisterUserNameDomainService;
use std::sync::Arc;
use tracing::{error, info};

#[async_trait]
pub trait AppService: Send + Sync {
    async fn register_user_name(
        &self,
        user_name: UnvalidatedUserName,
    ) -> Result<(), RegisterUserNameError>;
}

pub struct AppServiceImpl {
    register_user_name_domain_service: Arc<dyn RegisterUserNameDomainService>,
}
impl AppServiceImpl {
    pub fn new(register_user_name_domain_service: Arc<dyn RegisterUserNameDomainService>) -> Self {
        AppServiceImpl {
            register_user_name_domain_service,
        }
    }
}
#[async_trait]
impl AppService for AppServiceImpl {
    async fn register_user_name(
        &self,
        user_name: UnvalidatedUserName,
    ) -> Result<(), RegisterUserNameError> {
        let result = self
            .register_user_name_domain_service
            .register_user_name(user_name.clone())
            .await;

        if let Err(err) = &result {
            error!(
                "Failed to register user name: {:?} name: {:?}",
                err, &user_name.0
            );
        } else {
            info!("Successfully registered user name: {:?}", &user_name.0);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {
        use super::*;
        use domain_model::register_user_name::err::{ServiceError, ValidationError};
        use mockall::{mock, predicate::*};

        mock! {
            pub DomainService {}
            #[async_trait]
            impl RegisterUserNameDomainService for DomainService {
                async fn register_user_name(
                    &self,
                    user_name: UnvalidatedUserName,
                ) -> Result<(), RegisterUserNameError>;
            }
        }

        #[tokio::test]
        async fn test_register_user_name() {
            let user_name = UnvalidatedUserName("user_name".to_string());
            let unvalidated_user_name = UnvalidatedUserName(user_name.0.clone());
            let mut domain_service = MockDomainService::new();
            domain_service
                .expect_register_user_name()
                .with(eq(unvalidated_user_name))
                .times(1)
                .returning(|_| Ok(()));
            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let result = app_service.register_user_name(user_name).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_register_user_name_validation_error() {
            let user_name = UnvalidatedUserName("word".to_string());
            let unvalidated_user_name = UnvalidatedUserName(user_name.0.clone());
            let mut domain_service = MockDomainService::new();
            domain_service
                .expect_register_user_name()
                .with(eq(unvalidated_user_name))
                .times(1)
                .returning(|_| {
                    Err(RegisterUserNameError::from(ValidationError(
                        "Name must not contain NG words".to_string(),
                    )))
                });
            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let result = app_service.register_user_name(user_name).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                RegisterUserNameError::from(ValidationError(
                    "Name must not contain NG words".to_string()
                ))
            );
        }
        #[tokio::test]
        async fn test_register_user_name_service_error() {
            let user_name = UnvalidatedUserName("word".to_string());
            let unvalidated_user_name = UnvalidatedUserName(user_name.0.clone());
            let mut domain_service = MockDomainService::new();
            domain_service
                .expect_register_user_name()
                .with(eq(unvalidated_user_name))
                .times(1)
                .returning(|_| {
                    Err(RegisterUserNameError::from(ServiceError(
                        "ServiceError".to_string(),
                    )))
                });
            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let result = app_service.register_user_name(user_name).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                RegisterUserNameError::from(ServiceError("ServiceError".to_string()))
            );
        }
    }
}
