use std::collections::BTreeMap;

use surrealdb::sql::{thing, Object, Value};

use crate::{todo_input::TodoInput, utils::parse_only_first, DB};

#[derive(Debug, Default)]
pub struct Todo {
    pub id: String,
    pub done: bool,
    pub priority: i64,
    pub title: String,
}

impl From<&Object> for Todo {
    fn from(p: &Object) -> Self {
        Self {
            id: p.get("id").unwrap().clone().as_string(),
            done: match p.get("done").unwrap().clone() {
                Value::True => true,
                Value::False => false,
                _ => false,
            },
            priority: p.get("priority").unwrap().clone().as_int(),
            title: p.get("title").unwrap().clone().as_string(),
        }
    }
}

impl Todo {
    pub async fn delete(&self, (ds, ses): &DB) -> anyhow::Result<()> {
        let sql = "DELETE $th";
        let vars: BTreeMap<String, Value> = [("th".into(), thing(&self.id)?.into())].into();
        ds.execute(sql, ses, Some(vars), true).await?;
        Ok(())
    }

    pub async fn merge(&mut self, (ds, ses): &DB, input: TodoInput) -> anyhow::Result<Self> {
        let sql = "UPDATE $th MERGE $data;";
        let data: BTreeMap<String, Value> = [
            ("title".into(), input.title.into()),
            ("done".into(), input.done.into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [
            ("th".into(), thing(&self.id)?.into()),
            ("data".into(), data.into()),
        ]
        .into();

        let res = ds.execute(sql, ses, Some(vars), true).await?;
        let obj = parse_only_first(&res)?;
        let new_todo: Todo = obj.into();
        Ok(new_todo)
    }

    pub async fn get_by_id<'a>((ds, ses): &DB, id: String) -> anyhow::Result<Self> {
        let sql = "SELECT * FROM $th;";

        let vars: BTreeMap<String, Value> = [("th".into(), thing(id.as_str())?.into())].into();

        let res = ds.execute(sql, ses, Some(vars), true).await?;
        let obj = parse_only_first(&res)?;
        Ok(obj.into())
    }
}
