use actix_web::{self, get, web::Data, HttpResponse, Responder};
use app_service::get_all_post::AppService as PostAppService;
use domain_model::get_all_post::model::Post;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PostDto {
    pub posted_user_name: String,
    pub posted_datetime: String,
    pub content: String,
}
impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            posted_user_name: post.posted_user_name.0,
            posted_datetime: post
                .posted_datetime
                .0
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
            content: post.content.0,
        }
    }
}

#[get("/post")]
pub async fn get_all_post(service: Data<Arc<dyn PostAppService>>) -> impl Responder {
    let result = service.get_all_post().await;
    match result {
        Ok(posts) => {
            let posts: Vec<PostDto> = posts.into_iter().map(PostDto::from).collect();
            HttpResponse::Ok().json(posts)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use async_trait::async_trait;
    use chrono::NaiveDateTime;
    use domain_model::get_all_post::{
        err::ServiceError,
        model::{Content, Post, PostedDatetime, PostedUserName},
    };
    use mockall::{predicate::*, *};

    mock! {
        pub PostAppService {}
        #[async_trait]
        impl PostAppService for PostAppService {
            async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError>;
        }
    }

    #[actix_web::test]
    async fn test_get_all_post() {
        let mut mock = MockPostAppService::new();
        mock.expect_get_all_post().times(1).returning(|| {
            Ok(vec![Post {
                posted_user_name: PostedUserName("test".to_string()),
                posted_datetime: PostedDatetime(
                    NaiveDateTime::parse_from_str("2021-01-01T00:00:00Z", "%Y-%m-%dT%H:%M:%SZ")
                        .unwrap(),
                ),
                content: Content("test".to_string()),
            }])
        });
        let arc_service: Arc<dyn PostAppService> = Arc::new(mock);

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(arc_service))
                .service(get_all_post),
        )
        .await;
        let req = test::TestRequest::get().uri("/post").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_all_post_internal_server_error() {
        let mut mock = MockPostAppService::new();
        mock.expect_get_all_post()
            .times(1)
            .returning(|| Err(ServiceError("error".to_string())));

        let arc_service: Arc<dyn PostAppService> = Arc::new(mock);

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(arc_service))
                .service(get_all_post),
        )
        .await;
        let req = test::TestRequest::get().uri("/post").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
