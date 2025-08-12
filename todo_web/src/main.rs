use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

// エラーをまとめるenumを定義する。
// actix_web::ResponseErrorとして使うため、deriveマクロでDebugを付与している必要がある。
#[derive(Error, Debug)]
enum MyError {}

// actix_web::ResponseErrorをMyErrorに実装する。
impl ResponseError for MyError {}

// MyErrorはactix_web::ResponseErrorを実装しているので、
// indexの戻り値にMyErrorを使うことができる。
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
