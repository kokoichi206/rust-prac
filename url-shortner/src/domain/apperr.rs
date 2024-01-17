use thiserror::Error;

// application で発送しうるエラーの定義。
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),

    #[error("NotFound, id is {0}")]
    NotFound(String),
}
