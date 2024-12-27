use domain_model::register_user_name::model::UnvalidatedUserName;
use domain_service::register_user_name::DomainService as RegisterUserNameDomainService;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

#[derive(Debug, Clone, PartialEq)]
pub struct UserNameDto(String);

#[cfg_attr(test, automock)]
pub trait AppService {
    fn register_user_name(&self, user_name: UserNameDto) -> Result<(), String>;
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
    fn register_user_name(&self, user_name: UserNameDto) -> Result<(), String> {
        // todo サービスエラーかバリデーションエラーかで分岐する
        // self.register_user_name_domain_service
        //     .register_user_name(UnvalidatedUserName(user_name.0))
        //     .map_err(|err| err.to_string());
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service_impl {}
}
