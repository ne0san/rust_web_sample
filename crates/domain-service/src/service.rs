use domain_model::{err::RegisterUserNameError, model::UnvalidatedUserName};

use crate::repository::{CheckUserNameNgWordRepository, CreateUserNameRepository};
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
    mod register_user_name_service {
        use super::*;
    }
}
