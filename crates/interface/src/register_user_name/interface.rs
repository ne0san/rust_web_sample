use std::str;

use actix_web::{self, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserNameDto {
    user_name: String,
}

#[post("/user")]
pub async fn register_user(user_name: web::Json<UserNameDto>) -> impl Responder {
    HttpResponse::Ok()
}
