use actix_web::{
    self, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use app_service::register_ng_word::AppService as RegisterNgWordAppService;
use domain_model::register_ng_word::err::RegisterNgWordError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct RegisterNgWordDto {
    ng_word: String,
    user_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RegisterNgWordErrorDto {
    pub error_code: u16,
    pub error_message: String,
}

impl From<RegisterNgWordError> for RegisterNgWordErrorDto {
    fn from(err: RegisterNgWordError) -> Self {
        match err {
            RegisterNgWordError::ValidationError(err) => RegisterNgWordErrorDto {
                error_code: 400,
                error_message: err.0,
            },
            RegisterNgWordError::ServiceError(_) => RegisterNgWordErrorDto {
                error_code: 500,
                error_message: "Internal Server Error".to_string(),
            },
        }
    }
}

#[post("/register-ng-word")]
pub async fn register_ng_word(
    ng_word: web::Json<RegisterNgWordDto>,
    service: Data<Arc<dyn RegisterNgWordAppService>>,
) -> impl Responder {
    let result = service
        .register_ng_word(&ng_word.ng_word, ng_word.user_id)
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => match err {
            RegisterNgWordError::ValidationError(_) => {
                HttpResponse::BadRequest().json(RegisterNgWordErrorDto::from(err))
            }
            RegisterNgWordError::ServiceError(_) => {
                HttpResponse::InternalServerError().json(RegisterNgWordErrorDto::from(err))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use async_trait::async_trait;
    use domain_model::register_ng_word::err::{ServiceError, ValidationError};
    use mockall::{predicate::*, *};

    mock! {
        pub RegisterNgWordAppService {}

        #[async_trait]
        impl RegisterNgWordAppService for RegisterNgWordAppService {
            async fn register_ng_word(&self, ng_word: &str, user_id: i32) -> Result<(), RegisterNgWordError>;
        }
    }

    #[actix_web::test]
    async fn test_register_ng_word_ok() {
        let mut service = MockRegisterNgWordAppService::new();
        service
            .expect_register_ng_word()
            .with(eq("test"), eq(1))
            .times(1)
            .returning(|_, _| Ok(()));

        let arc_service: Arc<dyn RegisterNgWordAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .service(register_ng_word)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register-ng-word")
            .set_json(&RegisterNgWordDto {
                ng_word: "test".to_string(),
                user_id: 1,
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_register_ng_word_validation_error() {
        let mut service = MockRegisterNgWordAppService::new();
        service
            .expect_register_ng_word()
            .with(eq("te"), eq(1))
            .times(1)
            .returning(|_, _| {
                Err(RegisterNgWordError::ValidationError(ValidationError(
                    "Validation Error".to_string(),
                )))
            });

        let arc_service: Arc<dyn RegisterNgWordAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .service(register_ng_word)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register-ng-word")
            .set_json(&RegisterNgWordDto {
                ng_word: "te".to_string(),
                user_id: 1,
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_register_ng_word_service_error() {
        let mut service = MockRegisterNgWordAppService::new();
        service
            .expect_register_ng_word()
            .with(eq("test"), eq(1))
            .times(1)
            .returning(|_, _| {
                Err(RegisterNgWordError::ServiceError(ServiceError(
                    "Service Error".to_string(),
                )))
            });

        let arc_service: Arc<dyn RegisterNgWordAppService> = Arc::new(service);

        let mut app = test::init_service(
            App::new()
                .service(register_ng_word)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register-ng-word")
            .set_json(&RegisterNgWordDto {
                ng_word: "test".to_string(),
                user_id: 1,
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
