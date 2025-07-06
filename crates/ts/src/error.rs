use native_db::db_type;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TsError {
    #[error("Database error: {0}")]
    DbError(#[from] db_type::Error),
}

pub type Result<T> = std::result::Result<T, TsError>;
