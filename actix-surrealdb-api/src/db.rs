use chrono::offset::Utc;
use types::*;

use crate::prelude::*;
use crate::DB;

const INVESTMENT: &str = "investment";

pub async fn add_inv(inv: &mut Investment) -> Result<Investment> {
    inv.id = None;
    inv.created_at = Some(Utc::now());
    inv.updated_at = Some(Utc::now());
    let created: Investment = DB.create(INVESTMENT).content(inv).await?;

    Ok(created)
}

pub async fn get_inv(id: String) -> Result<Investment> {
    let th = id.split_once(':').unwrap();
    let rec: Investment = DB.select(th).await?;

    Ok(rec)
}

pub async fn delete_inv(id: String) -> Result<AffectedRows> {
    let th = id.split_once(':').unwrap();
    let _rec: Record = DB.delete(th).await?;

    Ok(AffectedRows { rows_affected: 1 })
}

pub async fn update_inv(inv: &mut Investment2) -> Result<Investment> {
    let (tb, id) = inv.id.split_once(':').unwrap();
    let sql = "UPDATE type::thing($tb, $id) SET name = $name, inv_type = $inv_type, inv_name = $inv_name, return_amount = $return_amount, return_rate = $return_rate, return_type = $return_type, inv_amount = $inv_amount, start_date = $start_date, end_date = $end_date, created_at = $created_at, updated_at = $updated_at;";
    inv.updated_at = Some(Utc::now());
    let mut response = DB
        .query(sql)
        .bind(("tb", tb))
        .bind(("id", id))
        .bind(("name", inv.name.clone()))
        .bind(("inv_type", inv.inv_type.clone()))
        .bind(("inv_name", inv.inv_name.clone()))
        .bind(("return_amount", inv.return_amount))
        .bind(("return_rate", inv.return_rate))
        .bind(("return_type", inv.return_type.clone()))
        .bind(("inv_amount", inv.inv_amount))
        .bind(("start_date", inv.start_date))
        .bind(("end_date", inv.end_date))
        .bind(("created_at", inv.created_at))
        .bind(("updated_at", inv.updated_at))
        .await?;

    let investment_updated = response
        .take::<Vec<Investment>>(0)?
        .into_iter()
        .next()
        .ok_or(Error::Generic("Failed to update record".into()))?;

    Ok(investment_updated)
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
