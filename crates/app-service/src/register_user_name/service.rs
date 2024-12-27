use domain_model::register_user_name::{err::RegisterUserNameError, model::UnvalidatedUserName};
use domain_service::register_user_name::DomainService as RegisterUserNameDomainService;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Debug, Clone, PartialEq)]
pub struct UserNameDto(String);

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterUserNameErrorDto {
    error_code: u16,
    error_message: String,
}
impl From<RegisterUserNameError> for RegisterUserNameErrorDto {
    fn from(err: RegisterUserNameError) -> Self {
        match err {
            RegisterUserNameError::ValidationError(err) => RegisterUserNameErrorDto {
                error_code: 400,
                error_message: err.0,
            },
            RegisterUserNameError::ServiceError(_) => RegisterUserNameErrorDto {
                error_code: 500,
                error_message: "Internal Server Error".to_string(),
            },
        }
    }
}

pub trait AppService {
    fn register_user_name(&self, user_name: UserNameDto) -> Result<(), RegisterUserNameErrorDto>;
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
impl AppService for AppServiceImpl {
    fn register_user_name(&self, user_name: UserNameDto) -> Result<(), RegisterUserNameErrorDto> {
        let unvalidated_user_name = UnvalidatedUserName(user_name.0.clone());
        let result = self
            .register_user_name_domain_service
            .register_user_name(unvalidated_user_name);
        if let Err(err) = &result {
            error!("Failed to register user name: {:?}", err);
        } else {
            info!("Successfully registered user name: {:?}", user_name.0);
        }
        result.map_err(|err| RegisterUserNameErrorDto::from(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {
        use super::*;
        use domain_model::register_user_name::err::ServiceError;
        use domain_model::register_user_name::err::ValidationError;
        use mockall::mock;
        use mockall::predicate::*;

        mock! {
            pub DomainService {}
            impl RegisterUserNameDomainService for DomainService {
                fn register_user_name(
                    &self,
                    user_name: UnvalidatedUserName,
                ) -> Result<(), RegisterUserNameError>;
            }
        }

        #[test]
        fn test_register_user_name() {
            let user_name = UserNameDto("user_name".to_string());
            let unvalidated_user_name = UnvalidatedUserName(user_name.0.clone());
            let mut domain_service = MockDomainService::new();
            domain_service
                .expect_register_user_name()
                .with(eq(unvalidated_user_name))
                .times(1)
                .returning(|_| Ok(()));
            let app_service = AppServiceImpl::new(Arc::new(domain_service));
            let result = app_service.register_user_name(user_name);
            assert!(result.is_ok());
        }

        #[test]
        fn test_register_user_name_validation_error() {
            let user_name = UserNameDto("word".to_string());
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
            let result = app_service.register_user_name(user_name);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                RegisterUserNameErrorDto {
                    error_code: 400,
                    error_message: "Name must not contain NG words".to_string(),
                }
            );
        }
        #[test]
        fn test_register_user_name_service_error() {
            let user_name = UserNameDto("word".to_string());
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
            let result = app_service.register_user_name(user_name);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                RegisterUserNameErrorDto {
                    error_code: 500,
                    error_message: "Internal Server Error".to_string(),
                }
            );
        }
    }
}
