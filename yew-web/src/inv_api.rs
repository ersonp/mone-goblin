use log::info;
use reqwasm::{http::Request, Error};
use std::collections::VecDeque;

use types::*;

#[allow(dead_code)]
const BASE_URL: &str = "http://localhost:8080";

pub async fn fetch_investments() -> Result<VecDeque<Investment2>, Error> {
    let response = Request::get(&format!("{BASE_URL}/invs")).send().await?;
    response.json().await
}

pub async fn create_investment(title: &str) -> Result<Investment2, Error> {
    let response = Request::post(&format!("{BASE_URL}/inv/{title}"))
        .send()
        .await?;

    response.json().await
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
