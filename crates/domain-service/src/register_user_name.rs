use crate::repository::RegisterUserNameRepository;
use domain_model::{
    err::{RegisterUserNameError, ValidationError},
    model::{UnvalidatedUserName, UserName},
};
use std::sync::Arc;

pub struct Service {
    register_user_name_repository: Arc<dyn RegisterUserNameRepository>,
}
impl Service {
    pub fn new(register_user_name_repository: Arc<dyn RegisterUserNameRepository>) -> Self {
        Self {
            register_user_name_repository,
        }
    }

    pub fn register_user_name(
        &self,
        user_name: UnvalidatedUserName,
    ) -> Result<(), RegisterUserNameError> {
        let user_name = UserName::new(&user_name.0)?;

        if self
            .register_user_name_repository
            .find_ng_word(&user_name)?
        {
            return Err(RegisterUserNameError::from(ValidationError(
                "Name must not contain NG words".to_string(),
            )));
        }

        let register_result = self
            .register_user_name_repository
            .create_user_name(&user_name);

        if let Err(err) = register_result {
            return Err(RegisterUserNameError::from(err));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mod register_user_name_service {
        use domain_model::err::ServiceError;

        use super::*;
        use crate::repository::MockRegisterUserNameRepository;

        #[test]
        fn test_register_user_name() {
            let mut register_user_name_repository = MockRegisterUserNameRepository::new();

            register_user_name_repository
                .expect_find_ng_word()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(false));

            register_user_name_repository
                .expect_create_user_name()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(()));

            let service = Service::new(Arc::new(register_user_name_repository));

            let result = service.register_user_name(UnvalidatedUserName("word".to_string()));

            assert_eq!(result, Ok(()));
        }

        #[test]
        fn test_register_user_name_too_long() {
            let mut register_user_name_repository = MockRegisterUserNameRepository::new();

            register_user_name_repository.expect_find_ng_word().times(0);

            register_user_name_repository
                .expect_create_user_name()
                .times(0);

            let service = Service::new(Arc::new(register_user_name_repository));

            let result =
                service.register_user_name(UnvalidatedUserName("veryverylongusern".to_string()));

            assert_eq!(
                result,
                Err(RegisterUserNameError::ValidationError(ValidationError(
                    "Username must be at most 16 characters long".to_string()
                )))
            );
        }

        #[test]
        fn test_register_user_name_too_short() {
            let mut register_user_name_repository = MockRegisterUserNameRepository::new();

            register_user_name_repository.expect_find_ng_word().times(0);

            register_user_name_repository
                .expect_create_user_name()
                .times(0);

            let service = Service::new(Arc::new(register_user_name_repository));

            let result = service.register_user_name(UnvalidatedUserName("sh".to_string()));

            assert_eq!(
                result,
                Err(RegisterUserNameError::ValidationError(ValidationError(
                    "Username must be at least 3 characters long".to_string()
                )))
            );
        }

        #[test]
        fn test_register_user_name_ng_word() {
            let mut register_user_name_repository = MockRegisterUserNameRepository::new();

            register_user_name_repository
                .expect_find_ng_word()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(true));

            register_user_name_repository
                .expect_create_user_name()
                .times(0)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(()));

            let service = Service::new(Arc::new(register_user_name_repository));

            let result = service.register_user_name(UnvalidatedUserName("word".to_string()));

            assert_eq!(
                result,
                Err(RegisterUserNameError::ValidationError(ValidationError(
                    "Name must not contain NG words".to_string()
                )))
            );
        }

        #[test]
        fn test_register_user_name_too_register_user_name_error() {
            let mut register_user_name_repository = MockRegisterUserNameRepository::new();

            register_user_name_repository
                .expect_find_ng_word()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(false));

            register_user_name_repository
                .expect_create_user_name()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Err(ServiceError("Already exists".to_string())));

            let service = Service::new(Arc::new(register_user_name_repository));

            let result = service.register_user_name(UnvalidatedUserName("word".to_string()));

            assert_eq!(
                result,
                Err(RegisterUserNameError::ServiceError(ServiceError(
                    "Already exists".to_string()
                )))
            );
        }

        #[test]
        fn test_register_user_name_check_ng_word_error() {
            let mut register_user_name_repository = MockRegisterUserNameRepository::new();

            register_user_name_repository
                .expect_find_ng_word()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Err(ServiceError("Service error".to_string())));

            register_user_name_repository
                .expect_create_user_name()
                .times(0);

            let service = Service::new(Arc::new(register_user_name_repository));

            let result = service.register_user_name(UnvalidatedUserName("word".to_string()));

            assert_eq!(
                result,
                Err(RegisterUserNameError::from(ServiceError(
                    "Service error".to_string(),
                )))
            );
        }
    }
}
