mod models;
mod api_actix;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPool;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("server started");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diatur di file .env");
    let PgPool = MySqlPool::connect(&database_url)
        .await
        .expect("Gagal membuat koneksi ke database");


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(PgPool.clone()))
            .service(api_actix::index)
            .service(api_actix::add_expenses)
            .service(api_actix::delete_expenses)
            .service(api_actix::get_expenses)
            .service(api_actix::get_expenses_by_id)
            .service(api_actix::get_expenses_by_last)
            .service(api_actix::get_expenses_by_day)
            .service(api_actix::get_log)
            .service(api_actix::get_log_by_last)
            .service(api_actix::get_psql_status)

    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}