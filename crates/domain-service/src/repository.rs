use domain_model::{err::ServiceError, model::UserName};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait CheckUserNameNgWordRepository {
    fn find_ng_word(&self, user_name: UserName) -> Result<bool, ServiceError>;
    // SELECT count(*) FROM ng_word WHERE ? LIKE CONCAT('%', word, '%'); みたいな感じで実装する
}

#[cfg_attr(test, automock)]
pub trait CreateUserNameRepository {
    fn create_user_name(&self, user_name: UserName) -> Result<(), ServiceError>;
}
