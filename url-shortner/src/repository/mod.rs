pub mod sqlite;

use std::error::Error;

pub trait Repository: Send + Sync + 'static {
    fn health(&self) -> Result<(), Box<dyn Error>>;
}
