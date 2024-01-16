pub mod label;
pub mod todo;

use thiserror::Error;

// repository で発送しうるエラーの定義。
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),

    #[error("NotFound, id is {0}")]
    NotFound(i32),
}
