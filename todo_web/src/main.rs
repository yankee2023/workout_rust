use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use r2d2::Pool;
use thiserror::Error;
use askama::Template;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// エラーをまとめるenumを定義する。
// actix_web::ResponseErrorとして使うため、deriveマクロでDebugを付与している必要がある。
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

// actix_web::ResponseErrorをMyErrorに実装する。
impl ResponseError for MyError {}

// MyErrorはactix_web::ResponseErrorを実装しているので、
// indexの戻り値にMyErrorを使うことができる。
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to create pool.");
    let conn = pool.get().expect("Failed to get connection from pool.");
    conn.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, text TEXT NOT NULL)", params![],).expect("Failed to create table `todo`.");
    HttpServer::new(move || App::new().service(index).app_data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
