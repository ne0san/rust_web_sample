use actix_web::{self, web::Data, App, HttpServer};
use app_service::{
    get_all_post::{
        AppService as GetAllPostAppService, AppServiceImpl as GetAllPostAppServiceImpl,
    },
    login::{AppService as LoginAppService, AppServiceImpl as LoginAppServiceImpl},
    post::{AppService as PostAppService, AppServiceImpl as PostAppServiceImpl},
    register_user_name::{
        AppService as RegisterUserNameAppService, AppServiceImpl as RegisterUserNameAppServiceImpl,
    },
};
use domain_service::{
    get_all_post::DomainServiceImpl as GetAllPostDomainService,
    login::DomainServiceImpl as LoginDomainService, post::DomainServiceImpl as PostDomainService,
    register_user_name::DomainServiceImpl as RegisterUserNameDomainService,
};
use dotenv::dotenv;
use infra::repository_impl::{
    GetAllPostRepositoryImpl, LoginRepositoryImpl, PostRepositoryImpl,
    RegisterUserNameRepositoryImpl,
};
use interface::{
    get_all_post::get_all_post, login::post_login, post::post_post, register_user_name::post_user,
};
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
        PostDomainService::new(Arc::new(PostRepositoryImpl::new(db_conn.clone()))),
    )));
    let login_app_service: Arc<dyn LoginAppService> = Arc::new(LoginAppServiceImpl::new(Arc::new(
        LoginDomainService::new(Arc::new(LoginRepositoryImpl::new(db_conn.clone()))),
    )));

    let get_all_post_service: Arc<dyn GetAllPostAppService> =
        Arc::new(GetAllPostAppServiceImpl::new(Arc::new(
            GetAllPostDomainService::new(Arc::new(GetAllPostRepositoryImpl::new(db_conn))),
        )));

    println!("Playground: http://localhost:8000");

    let factory = move || {
        App::new()
            .service(post_user)
            .app_data(Data::new(register_user_name_app_service.clone()))
            .service(post_post)
            .app_data(Data::new(post_app_service.clone()))
            .service(post_login)
            .app_data(Data::new(login_app_service.clone()))
            .service(get_all_post)
            .app_data(Data::new(get_all_post_service.clone()))
    };
    // ローカルサーバー
    HttpServer::new(factory)
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
