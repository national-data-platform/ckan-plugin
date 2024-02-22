use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use tokio_postgres::{NoTls, Error};
use std::env;
use dotenv::dotenv;

async fn query_db() -> Result<Vec<String>, Error> {
    dotenv().ok();
    let database_url = format!(
        "host={} user={} password={} dbname={} port={}",
        env::var("DB_HOST").unwrap(),
        env::var("DB_USER").unwrap(),
        env::var("DB_PASSWORD").unwrap(),
        env::var("DB_DATABASE").unwrap(),
        env::var("DB_PORT").unwrap()
    );
    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT name FROM users", &[])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect();

    Ok(rows)
}

async fn index() -> impl Responder {
    match query_db().await {
        Ok(names) => HttpResponse::Ok().json(names),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


