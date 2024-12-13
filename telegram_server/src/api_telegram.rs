use std::env;
use dotenv::dotenv;
use reqwest::Client;
use crate::models;
use teloxide::types::ChatId;
use serde_json::Value;


pub async fn cmd_start() -> String {
    let server_status = get_status_server().await;

    return format!("
Server Status
Teloxide  : Online
Postgre   : {}
Actix Web : {}
ketik /help untuk bantuan
", server_status, server_status)


}


pub async fn add_expenses(product: &str, amount: i32, user_id: &ChatId) -> String{
    dotenv().ok();
    let base_url = env::var("URL_SERVER").expect("URL_SERVER must be set");
    let url = format!("{}/expenses/add", base_url);
    let expense = models::AddExpense {
        name: product.to_string(),
        amount,
        user_id: user_id.to_string(),
    };
    let client = Client::new();
    let response = client.post(url)
        .json(&expense)
        .send()
        .await;
    match response {
        Ok(res) => {
            if res.status().is_success() {
                return format!("{} Telah Ditambahkan Dengan Harga {}", &product, amount);
            } else {
                return format!("Gagal Menambahkan Data");
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}


pub async fn delete_expenses(id: i32) -> String{
    dotenv().ok();
    let base_url = env::var("URL_SERVER").expect("URL_SERVER must be set");
    let url = format!("{}/expenses/delete", base_url);
    let expense = models::DeleteExpense {
        id
    };
    let client = Client::new();
    let response = client.delete(url)
        .json(&expense)
        .send()
        .await;
    match response {
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap_or_else(|_| "Gagal membaca response".to_string());
            if status.is_success() {
                format!("{}", &body)
            } else if status.is_client_error(){
                format!("{}", &body)
            } else {
                format!("Gagal Menghapus Data")
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub async fn get_expanses_by_last(amount: i32) -> String{
    dotenv().ok();
    let base_url = env::var("URL_SERVER").expect("URL_SERVER must be set");
    let url = format!("{}/expenses/last/{}", base_url, amount);
    let expense = models::LastExpense {
        amount,
    };
    let client = Client::new();
    let response = client.get(url)
        .json(&expense)
        .send()
        .await;
    match response {
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap_or_else(|_| "Gagal membaca response".to_string());
            if status.is_success() {
                let data: Vec<Value> = serde_json::from_str(&body).unwrap();
                let mut hasil = format!("Berikut adalah {} data pengeluaranmu terakhir:\n\n", amount);
                for item in data.iter().rev() {
                    let id = item["id"].as_i64().unwrap_or(0);
                    let name = item["name"].as_str().unwrap_or("unknown");
                    let amount = item["amount"].as_i64().unwrap_or(0);
                    let created_at = item["created_at"].as_str().unwrap_or("unknown");
                    let datetime = chrono::DateTime::parse_from_rfc3339(created_at).unwrap();
                    let created_at = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                    hasil.push_str(&format!(
                        "ID  : {}\nName     : {}\nAmount : {}\nCreated  : {}\n\n",
                        id, name, amount, created_at
                    ));
                }
                hasil
            } else {
                format!("{}", &body)
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub async fn get_expanses_by_day(amount: i32) -> String{
    dotenv().ok();
    let base_url = env::var("URL_SERVER").expect("URL_SERVER must be set");
    let url = format!("{}/expenses/day/{}", base_url, amount);
    let expense = models::LastExpense {
        amount,
    };
    let client = Client::new();
    let response = client.get(url)
        .json(&expense)
        .send()
        .await;
    match response {
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap_or_else(|_| "Gagal membaca response".to_string());
            if status.is_success() {
                let data: Vec<Value> = serde_json::from_str(&body).unwrap();
                let mut hasil = format!("Berikut adalah data pengeluaranmu {} hari terakhir:\n\n", amount);
                for item in data {
                    let id = item["id"].as_i64().unwrap_or(0);
                    let name = item["name"].as_str().unwrap_or("unknown");
                    let amount = item["amount"].as_i64().unwrap_or(0);
                    let created_at = item["created_at"].as_str().unwrap_or("unknown");
                    let datetime = chrono::DateTime::parse_from_rfc3339(created_at).unwrap();
                    let created_at = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                    hasil.push_str(&format!(
                        "ID  : {}\nName     : {}\nAmount : {}\nCreated  : {}\n\n",
                        id, name, amount, created_at
                    ));
                }
                hasil
            } else {
                format!("{}", &body)
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub async fn get_expanses_by_id(amount: i32) -> String{
    dotenv().ok();
    let base_url = env::var("URL_SERVER").expect("URL_SERVER must be set");
    let url = format!("{}/expenses/id/{}", base_url, amount);
    let expense = models::LastExpense {
        amount,
    };
    let client = Client::new();
    let response = client.get(url)
        .json(&expense)
        .send()
        .await;
    match response {
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap_or_else(|_| "Gagal membaca response".to_string());
            if status.is_success() {
                let data: Vec<Value> = serde_json::from_str(&body).unwrap();
                let mut hasil = String::from("Berikut adalah data pengeluaranmu:\n\n");
                for item in data.iter().rev() {
                    let id = item["id"].as_i64().unwrap_or(0);
                    let name = item["name"].as_str().unwrap_or("unknown");
                    let amount = item["amount"].as_i64().unwrap_or(0);
                    let created_at = item["created_at"].as_str().unwrap_or("unknown");
                    let datetime = chrono::DateTime::parse_from_rfc3339(created_at).unwrap();
                    let created_at = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                    hasil.push_str(&format!(
                        "ID  : {}\nName     : {}\nAmount : {}\nCreated  : {}\n\n",
                        id, name, amount, created_at
                    ));
                }
                hasil
            } else {
                format!("{}", &body)
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}


pub async fn get_status_server() -> String {
    dotenv().ok();
    let base_url = env::var("URL_SERVER").expect("URL_SERVER must be set");
    let url = format!("{}/psql/status", base_url);
    let client = Client::new();
    let response = client.get(url)
        .send()
        .await;
    match response {
        Ok(res) =>{
            if res.status().is_success() {
                "Online".to_string()
            } else {
                "Offline".to_string()
            }

        }
        Err(e) => format!("Offline {}", e),
    }
}