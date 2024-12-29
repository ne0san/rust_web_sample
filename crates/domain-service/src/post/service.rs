use super::repository::PostRepository;
use async_trait::async_trait;
use domain_model::post::{
    err::{PostError, ValidationError},
    model::Post,
};
use std::sync::Arc;

#[async_trait]
pub trait DomainService: Send + Sync {
    async fn post(&self, post: Post) -> Result<(), PostError>;
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
    async fn post(&self, post: Post) -> Result<(), PostError> {
        let user_exists = self.post_repository.user_exists(&post.user_name).await?;
        if !user_exists {
            return Err(PostError::from(ValidationError(
                "User does not exist".to_string(),
            )));
        }
        let ng_words = self.post_repository.find_all_ng_word().await?;

        // NGワードを全て、同じ文字数の*に置き換える
        let content = Post {
            user_name: post.user_name.clone(),
            content: ng_words
                .iter()
                .fold(post.content.clone(), |content, ng_word| {
                    content.replace(ng_word.value(), "*".repeat(ng_word.value().len()).as_str())
                }),
        };

        let result = self.post_repository.create_post(&content).await;
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
        use domain_model::post::{self, model::UserName};

        use crate::post::repository::MockPostRepository;

        use super::*;

        #[tokio::test]
        async fn test_post() {
            let user_name = UserName::new("user_name").unwrap();
            let post = Post {
                user_name: user_name.clone(),
                content: "content".to_string(),
            };
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
            let result = domain_service_impl.post(post).await;
            assert!(result.is_ok());
        }
    }
}
