use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Investment {
    pub id: Option<Thing>,
    pub inv_name: String,
    pub inv_type: String,
    pub return_rate: i32,
    pub return_rate_type: String,
    pub inv_amount: i32,
    pub return_amount: i32,
    pub name: String,
    pub start_date: DateTime<Local>,
    pub end_date: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}