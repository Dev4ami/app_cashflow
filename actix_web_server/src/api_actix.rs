use std::env;
use actix_web::{delete, get, HttpResponse, post, Responder, web};
use chrono::Local;
use dotenv::dotenv;
use sqlx::{MySqlPool};
use crate::models;

#[get("/")]
pub async fn index() -> impl Responder {
    format!("Hello World")
}


#[get("/expenses/all")]
pub async fn get_expenses(db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("GET", "/expenses/all").await;

    let query = "SELECT id, name, amount, user_id, created_at FROM data";
    match sqlx::query_as::<_, models::Expense>(query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}

#[get("/expenses/id/{id}")]
pub async fn get_expenses_by_id(id: web::Path<String>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("GET", &format!("/expenses/id/{}", id)).await;

    let query = format!("SELECT id, name, amount, user_id, created_at FROM data WHERE id = {}", id);
    match sqlx::query_as::<_, models::Expense>(&query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}

#[get("/expenses/last/{last}")]
pub async fn get_expenses_by_last(last: web::Path<String>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("GET", &format!("/expenses/last/{}", last)).await;
    let query = format!("SELECT id, name, amount, user_id, created_at From data ORDER BY created_at DESC limit {}", last);
    match sqlx::query_as::<_, models::Expense>(&query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}


#[get("/expenses/day/{day}")]
pub async fn get_expenses_by_day(day: web::Path<String>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("GET", &format!("/expenses/day/{}", day)).await;

    let query = format!("SELECT * FROM data WHERE created_at >= NOW() - INTERVAL '{} days'", day);
    match sqlx::query_as::<_, models::Expense>(&query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}

#[post("/expenses/add")]
pub async fn add_expenses(data: web::Json<models::InputDataExpenses>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("POST", "/expenses/add").await;
    let query = "INSERT INTO data (name, amount, user_id) VALUES (?, ?, ?)";
    let result = sqlx::query(query)
        .bind(&data.name)
        .bind(data.amount)
        .bind(&data.user_id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => format!("{} Ditambahkan Dengan Harga {}", data.name, data.amount),
        Err(e) => format!("Gagal menambahkan data: {}", e),
    }
}

#[delete("/expenses/delete")]
pub async fn delete_expenses(data: web::Json<models::DeleteDataExpenses>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("DELETE", "/expenses/delete").await;
    let query_id = "SELECT COUNT(*) FROM data WHERE id = ?";
    let check_result = sqlx::query_scalar::<_, i64>(query_id)
        .bind(&data.id)
        .fetch_one(db_pool.get_ref())
        .await;

    match check_result {
        Ok(count) if count > 0 => {
            let query = "DELETE FROM data where id = ?";
            let result = sqlx::query(query)
                .bind(&data.id)
                .execute(db_pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().body(format!("Data Dengan id {} Telah Dihapus", data.id)),
                Err(e) => HttpResponse::InternalServerError().body(format!("Gagal menghapus data: {}", e)),
            }
        }
        Ok(_) => HttpResponse::BadRequest().body(format!("Data Dengan id {} Tidak Ditemukan", data.id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Gagal Mendapatkan Data: {}", e)),
    }
}


async fn add_log(method: &str, location: &str){
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diatur di file .env");
    let db_pool = MySqlPool::connect(&database_url)
        .await
        .expect("Gagal membuat koneksi ke database");
    let query = "INSERT INTO log_request (location) VALUES (?)";
    sqlx::query(query)
        .bind(location)
        .execute(&db_pool)
        .await.unwrap();
    let time = Local::now();
    println!("[{}] http://localhost:8080{} -> {}", method, location, time);
}

#[get("/log/all")]
pub async fn get_log(db_pool: web::Data<MySqlPool>) -> impl Responder {
    
    add_log("GET", "/log/all").await;
    let query = "SELECT id, location, created_at FROM log_request";
    match sqlx::query_as::<_, models::LogData>(query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}

#[get("/log/last/{last}")]
pub async fn get_log_by_last(last: web::Path<String>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("GET", &format!("/log/last/{}", last)).await;
    let query = format!("SELECT id, location, created_at From log_request ORDER BY created_at DESC limit {}", last);
    match sqlx::query_as::<_, models::LogData>(&query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}

#[get("/psql/status")]
pub async fn get_psql_status(db_pool: web::Data<MySqlPool>) -> impl Responder {
    add_log("GET", &format!("/psql/status")).await;
    let query = format!("SELECT id, location, created_at From log_request limit 1");
    match sqlx::query_as::<_, models::LogData>(&query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil data.")
        }
    }
}

