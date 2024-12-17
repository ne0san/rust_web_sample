use actix_web::{self, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sea_orm::*;
use std::env;
use tracing::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

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
    let db = Database::connect(opt)
        .await
        .expect("Fail to Connect Database");

    println!("Playground: http://localhost:8000");

    let factory = move || App::new().service(echo).service(hello);
    // ローカルサーバー
    HttpServer::new(factory)
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
