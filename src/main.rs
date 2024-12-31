use actix_web::{self, web::Data, App, HttpServer};
use app_service::{
    post::{AppService as PostAppService, AppServiceImpl as PostAppServiceImpl},
    register_user_name::{
        AppService as RegisterUserNameAppService, AppServiceImpl as RegisterUserNameAppServiceImpl,
    },
};
use domain_service::{
    post::DomainServiceImpl as PostDomainService,
    register_user_name::DomainServiceImpl as RegisterUserNameDomainService,
};
use dotenv::dotenv;
use infra::repository_impl::{PostRepositoryImpl, RegisterUserNameRepositoryImpl};
use interface::{post::post_post, register_user_name::post_user};
use sea_orm::*;
use std::{env, sync::Arc};
use tracing::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv適用
    dotenv().ok();

    // ロガー設定
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let mut opt = ConnectOptions::new(env::var("DATABASE_URL").expect("DATABASE_URL is not set"));

    // sqlxのlog出力を設定
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let db_conn = Database::connect(opt)
        .await
        .expect("Fail to Connect Database");

    let register_user_name_app_service: Arc<dyn RegisterUserNameAppService> = Arc::new(
        RegisterUserNameAppServiceImpl::new(Arc::new(RegisterUserNameDomainService::new(
            Arc::new(RegisterUserNameRepositoryImpl::new(db_conn.clone())),
        ))),
    );
    let post_app_service: Arc<dyn PostAppService> = Arc::new(PostAppServiceImpl::new(Arc::new(
        PostDomainService::new(Arc::new(PostRepositoryImpl::new(db_conn))),
    )));

    println!("Playground: http://localhost:8000");

    let factory = move || {
        App::new()
            .service(post_user)
            .app_data(Data::new(register_user_name_app_service.clone()))
            .service(post_post)
            .app_data(Data::new(post_app_service.clone()))
    };
    // ローカルサーバー
    HttpServer::new(factory)
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
