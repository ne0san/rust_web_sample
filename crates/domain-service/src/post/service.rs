use super::repository::PostRepository;
use async_trait::async_trait;
use domain_model::post::{
    err::{PostError, ValidationError},
    model::{Post, UnvalidatedPost, UserName},
};
use std::sync::Arc;

#[async_trait]
pub trait DomainService: Send + Sync {
    async fn post(&self, post: UnvalidatedPost) -> Result<(), PostError>;
}

pub struct DomainServiceImpl {
    post_repository: Arc<dyn PostRepository>,
}
impl DomainServiceImpl {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        DomainServiceImpl { post_repository }
    }
}
#[async_trait]
impl DomainService for DomainServiceImpl {
    async fn post(&self, post: UnvalidatedPost) -> Result<(), PostError> {
        let user_name = UserName::new(&post.user_name.0)?;
        let user_exists = self.post_repository.user_exists(&user_name).await?;
        if !user_exists {
            return Err(PostError::from(ValidationError(
                "User does not exist".to_string(),
            )));
        }
        let ng_words = self.post_repository.find_all_ng_word().await?;

        // NGワードを全て、同じ文字数の*に置き換える
        let content = ng_words
            .iter()
            .fold(post.content.clone(), |content, ng_word| {
                content.replace(ng_word.value(), "*".repeat(ng_word.value().len()).as_str())
            });

        let post = Post::new(user_name.value().to_string(), content)?;

        let result = self.post_repository.create_post(&post).await;
        if let Err(err) = result {
            return Err(PostError::from(err));
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mod domain_service_impl {
        use domain_model::post::{
            self,
            model::{Post, UnvalidatedUserName},
        };

        use crate::post::repository::MockPostRepository;

        use super::*;

        #[tokio::test]
        async fn test_post() {
            let uv_user_name = UnvalidatedUserName("user_name".to_string());
            let user_name = post::model::UserName::new(&uv_user_name.0).unwrap();
            let uv_post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };
            let post = post::model::Post::new(uv_user_name.0, "content".to_string()).unwrap();
            let mut post_repository = MockPostRepository::new();
            post_repository
                .expect_user_exists()
                .with(eq(user_name))
                .times(1)
                .returning(|_| Ok(true));

            post_repository
                .expect_find_all_ng_word()
                .times(1)
                .returning(|| Ok(vec![]));

            post_repository
                .expect_create_post()
                .with(eq(post.clone()))
                .times(1)
                .returning(|_| Ok(()));

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(uv_post).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_post_invalid_user_name() {
            let uv_user_name = UnvalidatedUserName("un".to_string());
            let post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };

            let post_repository = MockPostRepository::new();

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(post).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                PostError::from(ValidationError(
                    "Username must be at least 3 characters long".to_string()
                ))
            );
        }

        #[tokio::test]
        async fn test_post_user_not_exists() {
            let uv_user_name = UnvalidatedUserName("user_name".to_string());
            let user_name = post::model::UserName::new(&uv_user_name.0).unwrap();
            let post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };
            let mut post_repository = MockPostRepository::new();
            post_repository
                .expect_user_exists()
                .with(eq(user_name))
                .times(1)
                .returning(|_| Ok(false));

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(post).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                PostError::from(ValidationError("User does not exist".to_string()))
            );
        }

        #[tokio::test]
        async fn test_post_ng_word() {
            let uv_user_name = UnvalidatedUserName("user_name".to_string());
            let user_name = post::model::UserName::new(&uv_user_name.0).unwrap();
            let uv_post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };
            let post = post::model::Post::new(uv_user_name.0, "*******".to_string()).unwrap();
            let mut post_repository = MockPostRepository::new();
            post_repository
                .expect_user_exists()
                .with(eq(user_name))
                .times(1)
                .returning(|_| Ok(true));

            post_repository
                .expect_find_all_ng_word()
                .times(1)
                .returning(|| Ok(vec![post::model::NgWord::new("content").unwrap()]));

            post_repository
                .expect_create_post()
                .with(eq(post.clone()))
                .times(1)
                .returning(|_| Ok(()));

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(uv_post).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_user_name_check_error() {
            let uv_user_name = UnvalidatedUserName("user_name".to_string());
            let user_name = post::model::UserName::new(&uv_user_name.0).unwrap();
            let uv_post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };
            let post = post::model::Post::new(uv_user_name.0, "content".to_string()).unwrap();
            let mut post_repository = MockPostRepository::new();
            post_repository
                .expect_user_exists()
                .with(eq(user_name))
                .times(1)
                .returning(|_| Err(post::err::ServiceError("ServiceError".to_string())));

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(uv_post).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                PostError::from(post::err::ServiceError("ServiceError".to_string()))
            );
        }

        #[tokio::test]
        async fn test_ng_word_check_error() {
            let uv_user_name = UnvalidatedUserName("user_name".to_string());
            let user_name = post::model::UserName::new(&uv_user_name.0).unwrap();
            let uv_post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };
            let post = post::model::Post::new(uv_user_name.0, "content".to_string()).unwrap();
            let mut post_repository = MockPostRepository::new();
            post_repository
                .expect_user_exists()
                .with(eq(user_name))
                .times(1)
                .returning(|_| Ok(true));

            post_repository
                .expect_find_all_ng_word()
                .times(1)
                .returning(|| {
                    Err(post::err::ServiceError(
                        "DomainServiceImpl error".to_string(),
                    ))
                });

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(uv_post).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                PostError::from(post::err::ServiceError(
                    "DomainServiceImpl error".to_string()
                ))
            );
        }

        #[tokio::test]
        async fn test_create_post_error() {
            let uv_user_name = UnvalidatedUserName("user_name".to_string());
            let user_name = post::model::UserName::new(&uv_user_name.0).unwrap();
            let uv_post = UnvalidatedPost {
                user_name: uv_user_name.clone(),
                content: "content".to_string(),
            };
            let post = post::model::Post::new(uv_user_name.0, "content".to_string()).unwrap();
            let mut post_repository = MockPostRepository::new();
            post_repository
                .expect_user_exists()
                .with(eq(user_name))
                .times(1)
                .returning(|_| Ok(true));

            post_repository
                .expect_find_all_ng_word()
                .times(1)
                .returning(|| Ok(vec![]));

            post_repository
                .expect_create_post()
                .with(eq(post.clone()))
                .times(1)
                .returning(|_| Err(post::err::ServiceError("ServiceError".to_string())));

            let domain_service_impl = DomainServiceImpl::new(Arc::new(post_repository));
            let result = domain_service_impl.post(uv_post).await;
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                PostError::from(post::err::ServiceError("ServiceError".to_string()))
            );
        }
    }
}
