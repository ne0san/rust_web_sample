use actix_web::{self, App, HttpServer};
use controller::{echo, hello};
use dotenv::dotenv;
use sea_orm::*;
use std::env;
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

    // let mut opt = ConnectOptions::new(env::var("DATABASE_URL").expect("DATABASE_URL not found"));

    // // sqlxのlog出力を設定
    // opt.sqlx_logging(true)
    //     .sqlx_logging_level(log::LevelFilter::Debug);
    // let db = Database::connect(opt)
    //     .await
    //     .expect("Fail to Connect Database");

    println!("Playground: http://localhost:8000");

    let factory = move || App::new().service(echo).service(hello);
    // ローカルサーバー
    HttpServer::new(factory)
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
