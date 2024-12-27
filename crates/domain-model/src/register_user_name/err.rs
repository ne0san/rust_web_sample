#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterUserNameError {
    ValidationError(ValidationError),
    ServiceError(ServiceError),
}
impl From<ValidationError> for RegisterUserNameError {
    fn from(e: ValidationError) -> Self {
        RegisterUserNameError::ValidationError(e)
    }
}
impl From<ServiceError> for RegisterUserNameError {
    fn from(e: ServiceError) -> Self {
        RegisterUserNameError::ServiceError(e)
    }
}
