use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::Db, error::Result, models::v1};

pub struct TrustedSystem {
    db: Db,
}

#[derive(Debug, Default)]
pub enum DbType {
    #[default]
    InMemory,
    File(PathBuf),
}

#[derive(Debug, Default)]
pub struct Config {
    pub db_type: DbType,
}

pub type Todo = v1::Todo;

impl TrustedSystem {
    pub fn new(config: Config) -> Result<Self> {
        let db = match config.db_type {
            DbType::InMemory => Db::in_memory(),
            DbType::File(path) => {
                if path.exists() {
                    Db::open(&path)
                } else {
                    Db::create(&path)
                }
            }
        }?;

        Ok(Self { db })
    }

    pub fn create_todo(&self, todo: NewTodo) -> Result<Todo> {
        self.db.create_todo(todo)
    }

    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        self.db.list_todos()
    }

    pub fn get_todo(&self, id: &str) -> Result<Option<Todo>> {
        self.db.get_todo(id)
    }

    pub fn complete_todo(&self, id: &str) -> Result<()> {
        if let Some(todo) = self.get_todo(id)? {
            let updated_todo = Todo {
                completed: true,
                ..todo.clone()
            };
            self.db.update_todo(todo, updated_todo)?;
        }
        Ok(())
    }

    pub fn delete_todo(&self, id: &str) -> Result<()> {
        if let Some(todo) = self.get_todo(id)? {
            self.db.delete_todo(todo)?;
        }
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct NewTodo {
    pub title: String,
    pub note: Option<String>,
    pub completed: bool,
}

impl From<NewTodo> for Todo {
    fn from(new_todo: NewTodo) -> Self {
        Todo {
            id: Uuid::now_v7().as_simple().to_string(),
            title: new_todo.title,
            note: new_todo.note,
            completed: new_todo.completed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test<F: FnOnce(TrustedSystem) -> ()>(test_fn: F) {
        let ts = TrustedSystem::new(Config {
            db_type: DbType::InMemory,
        })
        .expect("failed to set up trusted system");
        test_fn(ts);
    }

    #[test]
    fn test_create_todo() {
        run_test(|ts| {
            let todo = ts
                .create_todo(NewTodo {
                    title: "Test Todo".to_string(),
                    note: Some("This is a test note".to_string()),
                    ..Default::default()
                })
                .expect("Failed to create todo");

            let todos = ts.list_todos().expect("Failed to list todos");
            assert_eq!(todos.len(), 1);
            assert_eq!(todos[0], todo);
        })
    }

    #[test]
    fn test_list_todos() {
        run_test(|ts| {
            ts.create_todo(NewTodo {
                title: "First Todo".to_string(),
                ..Default::default()
            })
            .expect("Failed to create first todo");

            ts.create_todo(NewTodo {
                title: "Second Todo".to_string(),
                ..Default::default()
            })
            .expect("Failed to create second todo");

            let todos = ts.list_todos().expect("Failed to list todos");
            assert_eq!(todos.len(), 2);
        });
    }

    #[test]
    fn test_complete_todo() {
        run_test(|ts| {
            let todo = ts
                .create_todo(NewTodo {
                    title: "Complete Me".to_string(),
                    ..Default::default()
                })
                .expect("Failed to create todo");

            ts.complete_todo(&todo.id).expect("Failed to complete todo");

            let todo = ts.get_todo(&todo.id).expect("Failed to get todo").unwrap();
            assert!(todo.completed);
        });
    }
}
