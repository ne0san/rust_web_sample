#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum LoginError {
    ValidationError(ValidationError),
    ServiceError(ServiceError),
}
impl From<ValidationError> for LoginError {
    fn from(e: ValidationError) -> Self {
        LoginError::ValidationError(e)
    }
}
impl From<ServiceError> for LoginError {
    fn from(e: ServiceError) -> Self {
        LoginError::ServiceError(e)
    }
}
