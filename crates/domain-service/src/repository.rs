use domain_model::{err::ServiceError, model::UserName};
pub trait CheckUserNameNgWordRepository {
    fn check_ng_word(&self, user_name: UserName) -> Result<bool, ServiceError>;
    // SELECT count(*) FROM ng_word WHERE ? LIKE CONCAT('%', word, '%'); みたいな感じで実装する
}

pub trait CreateUserNameRepository {
    fn create_user_name(&self, user_name: UserName) -> Result<(), ServiceError>;
}
