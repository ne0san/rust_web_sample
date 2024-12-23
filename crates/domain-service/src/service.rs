use crate::repository::{CheckUserNameNgWordRepository, CreateUserNameRepository};
use domain_model::{
    err::RegisterUserNameError,
    model::{UnvalidatedUserName, UserName},
};
use std::sync::Arc;

pub struct RegisterUserNameService {
    check_user_name_ng_word_repository: Arc<dyn CheckUserNameNgWordRepository>,
    create_user_name_repository: Arc<dyn CreateUserNameRepository>,
}
impl RegisterUserNameService {
    pub fn new(
        check_user_name_ng_word_repository: Arc<dyn CheckUserNameNgWordRepository>,
        create_user_name_repository: Arc<dyn CreateUserNameRepository>,
    ) -> Self {
        Self {
            check_user_name_ng_word_repository,
            create_user_name_repository,
        }
    }

    pub fn register_user_name(
        &self,
        user_name: UnvalidatedUserName,
    ) -> Result<(), RegisterUserNameError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mod register_user_name_service {
        use crate::repository::{MockCheckUserNameNgWordRepository, MockCreateUserNameRepository};

        use super::*;

        fn service_build_with_mock(
            check_user_name_ng_word_repository: Arc<dyn CheckUserNameNgWordRepository>,
            create_user_name_repository: Arc<dyn CreateUserNameRepository>,
        ) -> RegisterUserNameService {
            RegisterUserNameService::new(
                check_user_name_ng_word_repository,
                create_user_name_repository,
            )
        }

        #[test]
        fn test_register_user_name() {
            let mut check_user_name_ng_word_repository = MockCheckUserNameNgWordRepository::new();
            check_user_name_ng_word_repository
                .expect_find_ng_word()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(false));

            let mut create_user_name_repository = MockCreateUserNameRepository::new();
            create_user_name_repository
                .expect_create_user_name()
                .times(1)
                .with(eq(UserName::new("word").unwrap()))
                .returning(|_| Ok(()));

            let service = service_build_with_mock(
                Arc::new(check_user_name_ng_word_repository),
                Arc::new(create_user_name_repository),
            );

            let result = service.register_user_name(UnvalidatedUserName("word".to_string()));

            assert_eq!(result, Ok(()));
        }

        #[test]
        #[ignore]
        fn test_register_user_name_ng_word() {
            // モックを
            // NGワードが含まれる場合
        }

        #[test]
        #[ignore]
        fn test_register_user_name_already_exists() {
            // モックを
            // 既に登録されている場合
        }

        #[test]
        #[ignore]
        fn test_register_user_name_too_long() {
            // モックを
            // 20文字を超える場合
        }

        #[test]
        #[ignore]
        fn test_register_user_name_too_short() {
            // モックを
            // 3文字未満の場合
        }

        #[test]
        #[ignore]
        fn test_register_user_name_too_check_ng_word_repository_error() {
            // モックを
            // NGワードリポジトリでのエラー
        }

        #[test]
        #[ignore]
        fn test_register_user_name_too_register_user_name_repository_error() {
            // モックを
            // ユーザーネームリポジトリでのエラー
        }
    }
}
