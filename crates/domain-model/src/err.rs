#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterUserNameError {
    ValidationError(ValidationError),
    ServiceError(ServiceError),
}
