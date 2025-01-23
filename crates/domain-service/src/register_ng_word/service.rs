use super::repository::RegisterNgWordRepository;
use async_trait::async_trait;
use domain_model::register_ng_word::{err::RegisterNgWordError, model::NgWord};
use std::sync::Arc;

#[async_trait]
pub trait DomainService: Send + Sync {
    async fn register_ng_word(
        &self,
        ng_word: &str,
        user_id: i32,
    ) -> Result<(), RegisterNgWordError>;
}

pub struct DomainServiceImpl {
    register_ng_word_repository: Arc<dyn RegisterNgWordRepository>,
}

impl DomainServiceImpl {
    pub fn new(register_ng_word_repository: Arc<dyn RegisterNgWordRepository>) -> Self {
        Self {
            register_ng_word_repository,
        }
    }
}

#[async_trait]
impl DomainService for DomainServiceImpl {
    async fn register_ng_word(
        &self,
        ng_word: &str,
        user_id: i32,
    ) -> Result<(), RegisterNgWordError> {
        let ng_word = NgWord::new(ng_word)?;
        self.register_ng_word_repository
            .register_ng_word(&ng_word, user_id)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain_model::register_ng_word::err::{ServiceError, ValidationError};
    use mockall::predicate::*;

    mod domain_service_impl {
        use super::*;
        use crate::register_ng_word::repository::MockRegisterNgWordRepository;

        #[tokio::test]
        async fn test_register_ng_word_success() {
            let mut mock = MockRegisterNgWordRepository::new();
            mock.expect_register_ng_word()
                .with(eq(NgWord::new("test").unwrap()), eq(1))
                .times(1)
                .returning(|_, _| Ok(()));

            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.register_ng_word("test", 1).await;

            assert!(result.is_ok());
            assert_eq!(result, Ok(()));
        }

        #[tokio::test]
        async fn test_register_ng_word_validation_error() {
            let mock = MockRegisterNgWordRepository::new();
            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.register_ng_word("te", 1).await;

            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(RegisterNgWordError::ValidationError(_))
            ));
        }

        #[tokio::test]
        async fn test_register_ng_word_service_error() {
            let mut mock = MockRegisterNgWordRepository::new();
            mock.expect_register_ng_word()
                .with(eq(NgWord::new("test").unwrap()), eq(1))
                .times(1)
                .returning(|_, _| Err(ServiceError("Database error".to_string())));

            let service = DomainServiceImpl::new(Arc::new(mock));
            let result = service.register_ng_word("test", 1).await;

            assert!(result.is_err());
            assert_eq!(
                result,
                Err(RegisterNgWordError::ServiceError(ServiceError(
                    "Database error".to_string()
                )))
            );
        }
    }
}
