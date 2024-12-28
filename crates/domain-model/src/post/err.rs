#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum PostError {
    ValidationError(ValidationError),
    ServiceError(ServiceError),
}
impl From<ValidationError> for PostError {
    fn from(e: ValidationError) -> Self {
        PostError::ValidationError(e)
    }
}
impl From<ServiceError> for PostError {
    fn from(e: ServiceError) -> Self {
        PostError::ServiceError(e)
    }
}
