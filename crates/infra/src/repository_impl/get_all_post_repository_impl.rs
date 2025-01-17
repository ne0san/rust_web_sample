use crate::entity::{m_user_name, t_post};
use async_trait::async_trait;
use domain_model::get_all_post::{
    err::ServiceError,
    model::{Content, Post, PostedDatetime, PostedUserName},
};
use domain_service::get_all_post::GetAllPostRepository;
use sea_orm::{entity::prelude::*, DatabaseConnection};
use tracing::error;

pub struct GetAllPostRepositoryImpl {
    db_conn: DatabaseConnection,
}
impl GetAllPostRepositoryImpl {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

#[async_trait]
impl GetAllPostRepository for GetAllPostRepositoryImpl {
    async fn get_all_post(&self) -> Result<Vec<Post>, ServiceError> {
        let result = t_post::Entity::find()
            .inner_join(m_user_name::Entity)
            .select_also(m_user_name::Entity)
            .all(&self.db_conn)
            .await;

        match result {
            Ok(posts) => {
                // Map the result to your domain model `Post`
                let posts: Vec<Post> = posts
                    .into_iter()
                    .map(|(post, user_name)| {
                        Post {
                            content: Content(post.content),
                            posted_user_name: PostedUserName(if let Some(user_name) = user_name {
                                user_name.name
                            } else {
                                "".to_string()
                            }),
                            posted_datetime: PostedDatetime(post.created_at.naive_local()), // Map other fields as necessary
                        }
                    })
                    .collect();
                Ok(posts)
            }
            Err(err) => {
                error!("Database query failed: {:?}", err);
                Err(ServiceError("Internal service error".to_string()))
            }
        }
    }
}
