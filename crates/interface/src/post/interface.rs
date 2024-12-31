use actix_web::{
    self, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use app_service::post::AppService as PostAppService;
use domain_model::post::{
    err::PostError,
    model::{UnvalidatedPost, UnvalidatedUserName},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct PostDto {
    user_name: String,
    content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PostErrorDto {
    pub error_code: u16,
    pub error_message: String,
}
impl From<PostError> for PostErrorDto {
    fn from(err: PostError) -> Self {
        match err {
            PostError::ValidationError(err) => PostErrorDto {
                error_code: 400,
                error_message: err.0,
            },
            PostError::ServiceError(_) => PostErrorDto {
                error_code: 500,
                error_message: "Internal Server Error".to_string(),
            },
        }
    }
}

#[post("/post")]
pub async fn post_post(
    post: web::Json<PostDto>,
    service: Data<Arc<dyn PostAppService>>,
) -> impl Responder {
    let user_name = UnvalidatedUserName(post.user_name.clone());
    let unvalidated_post = UnvalidatedPost {
        user_name,
        content: post.content.clone(),
    };
    let result = service.post(unvalidated_post);
    match result.await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => match err {
            PostError::ValidationError(_) => {
                HttpResponse::BadRequest().json(PostErrorDto::from(err))
            }
            PostError::ServiceError(_) => {
                HttpResponse::InternalServerError().json(PostErrorDto::from(err))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use async_trait::async_trait;
    use domain_model::post::err::{PostError, ServiceError, ValidationError};
    use mockall::{predicate::*, *};

    mock! {
        pub PostAppService {}
        #[async_trait]
        impl PostAppService for PostAppService {
            async fn post(
                &self,
                post: UnvalidatedPost,
            ) -> Result<(), PostError>;
        }
    }

    #[actix_web::test]
    async fn test_post_post_ok() {
        let mut mock_service = MockPostAppService::new();
        mock_service
            .expect_post()
            .with(eq(UnvalidatedPost {
                user_name: UnvalidatedUserName("user_name".to_string()),
                content: "content".to_string(),
            }))
            .times(1)
            .returning(|_| Ok(()));
        let arc_service: Arc<dyn PostAppService> = Arc::new(mock_service);

        let mut app = test::init_service(
            App::new()
                .service(post_post)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/post")
            .set_json(&PostDto {
                user_name: "user_name".to_string(),
                content: "content".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_post_validation_error() {
        let mut mock_service = MockPostAppService::new();
        mock_service
            .expect_post()
            .with(eq(UnvalidatedPost {
                user_name: UnvalidatedUserName("user_name".to_string()),
                content: "content".to_string(),
            }))
            .times(1)
            .returning(|_| {
                Err(PostError::ValidationError(ValidationError(
                    "Validation Error".to_string(),
                )))
            });

        let arc_service: Arc<dyn PostAppService> = Arc::new(mock_service);

        let mut app = test::init_service(
            App::new()
                .service(post_post)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/post")
            .set_json(&PostDto {
                user_name: "user_name".to_string(),
                content: "content".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_post_post_service_error() {
        let mut mock_service = MockPostAppService::new();
        mock_service
            .expect_post()
            .with(eq(UnvalidatedPost {
                user_name: UnvalidatedUserName("user_name".to_string()),
                content: "content".to_string(),
            }))
            .times(1)
            .returning(|_| {
                Err(PostError::ServiceError(ServiceError(
                    "Service Error".to_string(),
                )))
            });
        let arc_service: Arc<dyn PostAppService> = Arc::new(mock_service);

        let mut app = test::init_service(
            App::new()
                .service(post_post)
                .app_data(web::Data::new(arc_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/post")
            .set_json(&PostDto {
                user_name: "user_name".to_string(),
                content: "content".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
