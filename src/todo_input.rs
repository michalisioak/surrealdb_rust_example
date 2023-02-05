use std::collections::BTreeMap;

use surrealdb::sql::Value;

use crate::{todo_model::Todo, utils::parse_only_first, DB};

#[derive(Debug, Default)]
pub struct TodoInput {
    pub done: bool,
    pub priority: i64,
    pub title: String,
}

impl TodoInput {
    pub async fn create_task(self, (ds, ses): &DB) -> anyhow::Result<Todo> {
        let sql = "CREATE task CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("title".into(), self.title.clone().into()),
            ("priority".into(), self.priority.into()),
            ("done".into(), self.done.into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let res = ds.execute(sql, ses, Some(vars), false).await?;

        let obj = parse_only_first(&res)?;

        Ok(obj.into())
    }
}
