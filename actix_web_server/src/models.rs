use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Local};

#[derive(Deserialize)]
pub struct  InputDataExpenses {
    pub user_id: String,
    pub name: String,
    pub amount: i32,
}

#[derive(Deserialize)]
pub struct DeleteDataExpenses {
    pub id: u64,
}


#[derive(Serialize, FromRow)]
pub struct Expense {
    pub id: u64,
    pub name: String,
    pub amount: i32,
    pub user_id: String,
    pub created_at: Option<DateTime<Local>>,
}

#[derive(Serialize, FromRow)]
pub struct LogData {
    pub id: u64,
    pub location: String,
    pub created_at: Option<DateTime<Local>>,
}