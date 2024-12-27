use domain_service::register_user_name::DomainService as RegisterUserNameDomainService;
use std::sync::Arc;

pub struct AppService {
    register_user_name_domain_service: Arc<dyn RegisterUserNameDomainService>,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_service {}
}
