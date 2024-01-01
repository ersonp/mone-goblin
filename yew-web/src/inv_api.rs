use reqwasm::{http::Request, Error};
use serde_json;
use std::collections::VecDeque;

use types::*;

#[allow(dead_code)]
const BASE_URL: &str = "http://localhost:8080";

pub async fn fetch_investments() -> Result<VecDeque<Investment2>, Error> {
    let response = Request::get(&format!("{BASE_URL}/invs")).send().await?;
    response.json().await
}

pub async fn create_investment(inv: String) -> Result<Investment2, Error> {
    let response = Request::post(&format!("{}/inv", BASE_URL))
        .header("Content-Type", "application/json")
        .body(inv) // Set the serialized JSON as the body
        .send()
        .await?;

    // Log the response body
    let response_body = response.text().await?;

    // Parse the response body as JSON
    let inv: Investment2 = serde_json::from_str(&response_body)?;

    Ok(inv)
}

pub async fn edit_investment(id: String) -> Result<AffectedRows, Error> {
    let response = Request::patch(&format!("{BASE_URL}/inv/{id}"))
        .send()
        .await?;

    response.json().await
}

pub async fn delete_investment(id: String) -> Result<AffectedRows, Error> {
    let response = Request::delete(&format!("{BASE_URL}/inv/{id}"))
        .send()
        .await?;

    response.json().await
}
