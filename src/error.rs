pub enum TaskError {
    NotFound(String),
    AlreadyCompleted,
    InvalidInput(String),
    StoreError(String),
}