use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Aircraft data unavailable: {0}")]
    DataUnavailable(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}
