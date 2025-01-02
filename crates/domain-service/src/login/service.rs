use super::repository::LoginRepository;
use async_trait::async_trait;
use domain_model::login::{
    err::{LoginError, ValidationError},
    model::UncheckedUserName,
};
use std::sync::Arc;

#[async_trait]
pub trait DomainService: Send + Sync {
    async fn login(&self, user: UncheckedUserName) -> Result<(), LoginError>;
}

pub struct DomainServiceImpl {
    login_repository: Arc<dyn LoginRepository>,
}
impl DomainServiceImpl {
    pub fn new(login_repository: Arc<dyn LoginRepository>) -> Self {
        Self { login_repository }
    }
}

#[async_trait]
impl DomainService for DomainServiceImpl {
    async fn login(&self, user: UncheckedUserName) -> Result<(), LoginError> {
        if !self.login_repository.user_exists(&user).await? {
            return Err(LoginError::from(ValidationError(
                "User not found".to_string(),
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mod domain_service_impl {
        use domain_model::login::err::ServiceError;

        use super::*;
        use crate::login::repository::MockLoginRepository;

        #[tokio::test]
        async fn test_login() {
            let mut mock = MockLoginRepository::new();
            mock.expect_user_exists()
                .with(eq(UncheckedUserName("test".to_string())))
                .times(1)
                .returning(|_| Ok(true));

            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.login(UncheckedUserName("test".to_string())).await;

            assert!(result.is_ok());
            assert_eq!(result, Ok(()));
        }
        #[tokio::test]
        async fn test_login_user_not_found() {
            let mut mock = MockLoginRepository::new();
            mock.expect_user_exists()
                .with(eq(UncheckedUserName("test".to_string())))
                .times(1)
                .returning(|_| Ok(false));

            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.login(UncheckedUserName("test".to_string())).await;

            assert!(result.is_err());
            assert_eq!(
                result,
                Err(LoginError::from(ValidationError(
                    "User not found".to_string()
                )))
            );
        }

        #[tokio::test]
        async fn test_login_error() {
            let mut mock = MockLoginRepository::new();
            mock.expect_user_exists()
                .with(eq(UncheckedUserName("test".to_string())))
                .times(1)
                .returning(|_| Err(ServiceError("error".to_string())));

            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.login(UncheckedUserName("test".to_string())).await;

            assert!(result.is_err());
            assert_eq!(
                result,
                Err(LoginError::from(ServiceError("error".to_string())))
            );
        }
    }
}
