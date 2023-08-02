use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Investment {
    pub id: Option<Thing>,
    pub inv_name: String,
    pub inv_type: String,
    pub return_rate: i32,
    pub return_rate_type: String,
    pub inv_amount: i32,
    pub return_amount: i32,
    pub name: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
// pub struct Investment2 {
//     pub id: String,
//     pub inv_name: String,
//     pub inv_type: String,
//     pub return_rate: i32,
//     pub return_rate_type: String,
//     pub inv_amount: i32,
//     pub return_amount: i32,
//     pub name: String,
//     pub start_date: Option<DateTime<Utc>>,
//     pub end_date: Option<DateTime<Utc>>,
//     pub created_at: Option<DateTime<Utc>>,
//     pub updated_at: Option<DateTime<Utc>>,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}
