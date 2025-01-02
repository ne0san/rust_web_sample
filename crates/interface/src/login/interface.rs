use actix_web::{
    self, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use app_service::login::AppService as LoginAppService;
use domain_model::login::{err::LoginError, model::UncheckedUserName};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct LoginDto {
    user_name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoginErrorDto {
    pub error_code: u16,
    pub error_message: String,
}
impl From<LoginError> for LoginErrorDto {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::ValidationError(err) => LoginErrorDto {
                error_code: 400,
                error_message: err.0,
            },
            LoginError::ServiceError(_) => LoginErrorDto {
                error_code: 500,
                error_message: "Internal Server Error".to_string(),
            },
        }
    }
}

#[post("/login")]
pub async fn post_login(
    user_name: web::Json<LoginDto>,
    service: Data<Arc<dyn LoginAppService>>,
) -> impl Responder {
    let unchecked_user_name = UncheckedUserName(user_name.user_name.clone());
    let result = service.login(unchecked_user_name);
    match result.await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => match err {
            LoginError::ValidationError(_) => {
                HttpResponse::BadRequest().json(LoginErrorDto::from(err))
            }
            LoginError::ServiceError(_) => {
                HttpResponse::InternalServerError().json(LoginErrorDto::from(err))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use async_trait::async_trait;
    use domain_model::login::err::{LoginError, ServiceError, ValidationError};
    use mockall::{predicate::*, *};

    mock! {
        pub LoginAppService {}

        #[async_trait]
        impl LoginAppService for LoginAppService {
            async fn login(&self, user_name: UncheckedUserName) -> Result<(), LoginError>;
        }
    }

    #[actix_web::test]
    async fn test_post_login_ok() {
        let mut service = MockLoginAppService::new();
        service
            .expect_login()
            .with(eq(UncheckedUserName("user_name".to_string())))
            .times(1)
            .returning(|_| Ok(()));

        let arc_service: Arc<dyn LoginAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .service(post_login)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&LoginDto {
                user_name: "user_name".to_string(),
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_login_validation_error() {
        let mut service = MockLoginAppService::new();
        service
            .expect_login()
            .with(eq(UncheckedUserName("user_name".to_string())))
            .times(1)
            .returning(|_| {
                Err(LoginError::ValidationError(ValidationError(
                    "Validation Error".to_string(),
                )))
            });

        let arc_service: Arc<dyn LoginAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .service(post_login)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&LoginDto {
                user_name: "user_name".to_string(),
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_post_login_service_error() {
        let mut service = MockLoginAppService::new();
        service
            .expect_login()
            .with(eq(UncheckedUserName("user_name".to_string())))
            .times(1)
            .returning(|_| {
                Err(LoginError::ServiceError(ServiceError(
                    "Service Error".to_string(),
                )))
            });

        let arc_service: Arc<dyn LoginAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .service(post_login)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&LoginDto {
                user_name: "user_name".to_string(),
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
