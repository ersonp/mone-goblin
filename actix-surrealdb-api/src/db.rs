use chrono::offset::Utc;
use surrealdb::sql::Thing;

use crate::prelude::*;
use crate::DB;
use types::*;

const INVESTMENT: &str = "investment";

pub async fn add_inv(inv: &mut Investment) -> Result<Investment> {
    inv.id = None;
    inv.created_at = Some(Utc::now());
    inv.updated_at = Some(Utc::now());
    let created: Vec<Investment> = DB.create(INVESTMENT).content(inv).await?;

    Ok(created.clone().pop().unwrap())
}

pub async fn get_inv(id: String) -> Result<Investment> {
    let th = id.split_once(':').unwrap();
    let rec: Option<Investment> = DB.select(th).await?;

    Ok(rec.unwrap())
}

pub async fn delete_inv(id: Thing) -> Result<Record> {
    let response_option: Option<Record> = DB.delete(id).await?;
    let response = response_option.ok_or(Error::Generic("Failed to delete record".into()))?;

    Ok(response)
}

pub async fn update_inv(inv: &mut Investment) -> Result<Investment> {
    let thing = match inv.id.clone() {
        Some(thing) => thing,
        None => return Err(Error::Generic("Failed to update record".into())),
    };
    let response_option: Option<Investment> = DB.update(thing).content(inv).await?;
    let response = response_option.ok_or(Error::Generic("Failed to update record".into()))?;

    Ok(response)
}

pub async fn get_all_invs() -> Result<Vec<Investment>> {
    // let tasks: Vec<Task> = DB.select(TASK).await?;

    // Ok(tasks)
    let sql = "SELECT * FROM type::table($table) ORDER BY created_at DESC;";

    let mut response = DB.query(sql).bind(("table", INVESTMENT)).await?;

    let tasks: Vec<Investment> = response.take(0)?;

    Ok(tasks)
}

/*
 * https://surrealdb.com/docs/surrealql/functions/type#thing
 * https://surrealdb.com/docs/surrealql/functions/script
 * https://stackoverflow.com/questions/36876570/return-first-item-of-vector
 * https://stackoverflow.com/questions/57707966/how-to-get-timestamp-of-the-current-date-and-time-in-rust
 */

/*
 * RAZÓN POR LA QUE USAR EL VERBO "PATCH" Y NO "PUT". VER:
 * https://www.abstractapi.com/guides/put-vs-patch
 * https://www.baeldung.com/cs/http-put-vs-patch
 * https://github.com/letsgetrusty/rsty-stack-example/tree/main/todo_api
 * https://www.google.com/search?q=difference+between+patch+and+put&oq=dif&aqs=chrome.0.69i59j69i57j35i39j69i60l2j69i65l3.2889j0j7&sourceid=chrome&ie=UTF-8
 *
 * SOBRE ok_or. VER:
 * Is there a way to simplify converting an Option into a Result without a macro?
 * https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
 */
