use actix_web::{get, post, web, App, HttpResponse, HttpServer, ResponseError, http::header};
use r2d2::Pool;
use thiserror::Error;
use askama::Template;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

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

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

// actix_web::ResponseErrorをMyErrorに実装する。
impl ResponseError for MyError {}

#[post("/add")]
async fn add_todo(params: web::Form<AddParams>, db: web::Data<r2d2::Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let connection = db.get()?;
    connection.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
    Ok(HttpResponse::SeeOther().header(header::LOCATION, "/").finish())
}

#[post("/delete")]
async fn delete_todo(params: web::Form<DeleteParams>, db: web::Data<r2d2::Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let connection = db.get()?;
    connection.execute("DELETE FROM todo WHERE id = ?", &[&params.id])?;
    Ok(HttpResponse::SeeOther().header(header::LOCATION, "/").finish())
}

// MyErrorはactix_web::ResponseErrorを実装しているので、
// indexの戻り値にMyErrorを使うことができる。
#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let connection = db.get()?;

    // SQL分をPrepared Statementに変換
    let mut statement = connection.prepare("SELECT id, text FROM todo")?;

    // Prepared StatementとなっているSQL文を実行し、結果をTodoEntryに変換
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    let html = IndexTemplate { entries };
    let response_body = html.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let connection = pool.get().expect("Failed to get the connection from pool.");
    connection.execute("CREATE TABLE IF NOT EXISTS todo (id INTEGER PRIMARY KEY AUTOINCREMENT, text TEXT NOT NULL)", params![],)
              .expect("Failed to create a table `todo`.");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
