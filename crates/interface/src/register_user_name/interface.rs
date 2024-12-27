use actix_web::{
    self, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use app_service::register_user_name::AppService as RegisterUserNameAppService;
use domain_model::register_user_name::{err::RegisterUserNameError, model::UnvalidatedUserName};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct UserNameDto {
    user_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RegisterUserNameErrorDto {
    pub error_code: u16,
    pub error_message: String,
}
impl From<RegisterUserNameError> for RegisterUserNameErrorDto {
    fn from(err: RegisterUserNameError) -> Self {
        match err {
            RegisterUserNameError::ValidationError(err) => RegisterUserNameErrorDto {
                error_code: 400,
                error_message: err.0,
            },
            RegisterUserNameError::ServiceError(_) => RegisterUserNameErrorDto {
                error_code: 500,
                error_message: "Internal Server Error".to_string(),
            },
        }
    }
}

#[post("/user")]
pub async fn post_user(
    user_name: web::Json<UserNameDto>,
    service: Data<Arc<dyn RegisterUserNameAppService>>,
) -> impl Responder {
    let user_name_dto = UnvalidatedUserName(user_name.user_name.clone());
    let result = service.register_user_name(user_name_dto);
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => match err {
            RegisterUserNameError::ValidationError(_) => {
                HttpResponse::BadRequest().json(RegisterUserNameErrorDto::from(err))
            }
            RegisterUserNameError::ServiceError(_) => {
                HttpResponse::InternalServerError().json(RegisterUserNameErrorDto::from(err))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use domain_model::register_user_name::err::{
        RegisterUserNameError, ServiceError, ValidationError,
    };
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub RegisterUserNameAppService {}
        impl RegisterUserNameAppService for RegisterUserNameAppService {
            fn register_user_name(
                &self,
                user_name: UnvalidatedUserName,
            ) -> Result<(), RegisterUserNameError>;
        }
    }

    #[actix_web::test]
    async fn test_register_user() {
        let user_name = UnvalidatedUserName("user_name".to_string());
        let user_name_dto = UserNameDto {
            user_name: user_name.0.clone(),
        };

        let mut service = MockRegisterUserNameAppService::new();
        service
            .expect_register_user_name()
            .with(eq(user_name))
            .times(1)
            .returning(|_| Ok(()));
        let arc_service: Arc<dyn RegisterUserNameAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(arc_service))
                .service(post_user),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&user_name_dto)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(test::read_body(resp).await, web::Bytes::from_static(b""));
    }

    #[actix_web::test]
    async fn test_register_user_validation_error() {
        let user_name = UnvalidatedUserName("user_name".to_string());
        let user_name_dto = UserNameDto {
            user_name: user_name.0.clone(),
        };

        let mut service = MockRegisterUserNameAppService::new();
        service
            .expect_register_user_name()
            .with(eq(user_name))
            .times(1)
            .returning(|_| {
                Err(RegisterUserNameError::from(ValidationError(
                    "Name must not contain NG words".to_string(),
                )))
            });
        let arc_service: Arc<dyn RegisterUserNameAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(arc_service))
                .service(post_user),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&user_name_dto)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
        assert_eq!(
            test::read_body(resp).await,
            web::Bytes::from_static(
                b"{\"error_code\":400,\"error_message\":\"Name must not contain NG words\"}"
            )
        );
    }

    #[actix_web::test]
    async fn test_register_user_service_error() {
        let user_name = UnvalidatedUserName("user_name".to_string());
        let user_name_dto = UserNameDto {
            user_name: user_name.0.clone(),
        };

        let mut service = MockRegisterUserNameAppService::new();
        service
            .expect_register_user_name()
            .with(eq(user_name))
            .times(1)
            .returning(|_| {
                Err(RegisterUserNameError::from(ServiceError(
                    "ServiceError".to_string(),
                )))
            });
        let arc_service: Arc<dyn RegisterUserNameAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(arc_service))
                .service(post_user),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&user_name_dto)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            test::read_body(resp).await,
            web::Bytes::from_static(
                b"{\"error_code\":500,\"error_message\":\"Internal Server Error\"}"
            )
        );
    }
}
