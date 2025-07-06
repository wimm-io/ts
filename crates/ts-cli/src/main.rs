use std::path::PathBuf;

use ts::{
    app::{self, NewTodo, TrustedSystem},
    error::Result,
};

fn main() -> Result<()> {
    let ts = TrustedSystem::new(app::Config {
        db_type: app::DbType::File(PathBuf::from("/Users/scott/Desktop/test.redb")),
        ..Default::default()
    })?;

    let id1 = ts.create_todo(NewTodo {
        title: "foo".to_string(),
        ..Default::default()
    })?;

    println!("created Todo: {id1}...");

    let id2 = ts.create_todo(NewTodo {
        title: "goof".to_string(),
        note: Some("bar".to_string()),
        ..Default::default()
    })?;

    println!("created Todo: {id2}...");

    println!("listing Todos:");
    for todo in ts.list_todos()? {
        println!("Todo: {todo:?}");
    }

    Ok(())
}
