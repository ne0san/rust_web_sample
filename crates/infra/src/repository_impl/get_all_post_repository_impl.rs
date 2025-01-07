use crate::entity::{m_post, m_user};
use async_trait::async_trait;
use domain_model::get_all_post::{err::ServiceError, model::Post};
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
