#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterNgWordError {
    ValidationError(ValidationError),
    ServiceError(ServiceError),
}
impl From<ValidationError> for RegisterNgWordError {
    fn from(e: ValidationError) -> Self {
        RegisterNgWordError::ValidationError(e)
    }
}
impl From<ServiceError> for RegisterNgWordError {
    fn from(e: ServiceError) -> Self {
        RegisterNgWordError::ServiceError(e)
    }
}
