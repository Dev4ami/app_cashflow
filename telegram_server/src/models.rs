use serde::Serialize;

#[derive(Serialize)]
pub struct AddExpense {
    pub name: String,
    pub amount: i32,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct DeleteExpense {
    pub id: i32
}

#[derive(Serialize)]
pub struct LastExpense {
    pub amount: i32
}