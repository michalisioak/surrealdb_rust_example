use surrealdb::sql::{Object, Value};

pub fn parse_only_first<'a>(res: &'a Vec<surrealdb::dbs::Response>) -> anyhow::Result<&'a Object> {
    let result = res
        .first()
        .ok_or_else(|| anyhow::anyhow!("Problem with database"))?
        .result
        .as_ref()
        .ok();

    match result {
        Some(Value::Array(arr)) => {
            let obj = arr.first().ok_or_else(|| anyhow::anyhow!("Not Found"))?;
            match obj {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow::anyhow!("A record was not an Object")),
            }
        }
        _ => Err(anyhow::anyhow!("Invalid Response")),
    }
}
