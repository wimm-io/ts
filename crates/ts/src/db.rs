use std::path::Path;

use native_db::{Builder, db_type};

use crate::{app::Todo, error::Result, models::get_models};

pub struct Db {
    inner: native_db::Database<'static>,
}

impl Db {
    pub fn in_memory() -> Result<Self> {
        Ok(Self {
            inner: Builder::new().create_in_memory(get_models())?,
        })
    }

    pub fn create(path: &Path) -> Result<Self> {
        Ok(Self {
            inner: Builder::new().create(get_models(), path)?,
        })
    }

    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self {
            inner: Builder::new().open(get_models(), path)?,
        })
    }

    pub fn create_todo<T: Into<Todo>>(&self, todo: T) -> Result<String> {
        let todo: Todo = todo.into();
        let id = todo.id.clone();
        let tx = self.inner.rw_transaction()?;
        tx.insert::<Todo>(todo)?;
        tx.commit()?;
        Ok(id)
    }

    pub fn get_todo(&self, id: &str) -> Result<Option<Todo>> {
        let tx = self.inner.r_transaction()?;
        Ok(tx.get().primary(id)?)
    }

    pub fn update_todo(&self, old: Todo, new: Todo) -> Result<()> {
        let tx = self.inner.rw_transaction()?;
        tx.update(old, new)?;
        tx.commit()?;
        Ok(())
    }

    pub fn delete_todo(&self, todo: Todo) -> Result<()> {
        let tx = self.inner.rw_transaction()?;
        tx.remove(todo)?;
        tx.commit()?;
        Ok(())
    }

    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        let tx = self.inner.r_transaction()?;
        let todos = tx
            .scan()
            .primary::<Todo>()?
            .all()?
            .collect::<db_type::Result<Vec<_>>>()?;

        Ok(todos)
    }
}
