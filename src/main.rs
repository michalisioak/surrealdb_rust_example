use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

mod todo_input;
mod todo_model;
mod utils;

pub type DB = (Datastore, Session);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db: &DB = &(
        Datastore::new("file:./temp.db").await?,
        Session::for_db("ns_name", "db_name"),
    );

    // --- Create
    let t1 = todo_input::TodoInput {
        done: false,
        priority: 10,
        title: "Task 01".to_owned(),
        ..Default::default()
    }
    .create_task(db)
    .await?;

    println!("t1: {:?}", t1);

    let mut t2 = todo_input::TodoInput {
        done: false,
        priority: 7,
        title: "Task 02".to_owned(),
        ..Default::default()
    }
    .create_task(db)
    .await?;
    println!("t2: {:?}", t2);

    // --- Merge
    t2 = t2
        .merge(
            db,
            todo_input::TodoInput {
                done: true,
                title: "Task 02 UPDATED".to_owned(),
                ..Default::default()
            },
        )
        .await?;
    println!("Updated Todo: {:?}", t2);

    // --- Get By Id
    let fetched_todo = todo_model::Todo::get_by_id(db, t2.id).await?;
    println!("Fetched Todo: {:?}", fetched_todo);

    // --- Delete
    t1.delete(db).await?;

    Ok(())
}
