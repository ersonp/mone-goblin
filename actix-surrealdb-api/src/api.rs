use actix_web::{
    delete,
    get,
    patch,
    post,
    web,
    web::{Json, Path},
    // HttpResponse,
};
use surrealdb::sql::Thing;

use crate::db::*;
use crate::prelude::*;
use types::*;

#[post("/inv")]
pub async fn create(inv: web::Json<Investment>) -> Result<Json<Investment>> {
    let mut inv = inv.into_inner();
    let todo = add_inv(&mut inv).await?;
    Ok(Json(todo))
}

#[get("/inv/{id}")]
pub async fn get(id: Path<String>) -> Result<Json<Investment>> {
    let task = get_inv(id.into_inner()).await?;

    Ok(Json(task))
}

#[patch("/inv")]
pub async fn update(inv: web::Json<Investment>) -> Result<Json<Investment>> {
    let mut inv = inv.into_inner();
    let updated = update_inv(&mut inv).await?;

    Ok(Json(updated))
}

#[delete("/inv")]
pub async fn delete(id: web::Json<Thing>) -> Result<Json<Record>> {
    let deleted = delete_inv(id.into_inner()).await?;

    Ok(Json(deleted))
}

#[get("/invs")]
pub async fn list() -> Result<Json<Vec<Investment>>> {
    let todos = get_all_invs().await?;
    Ok(Json(todos))
}
