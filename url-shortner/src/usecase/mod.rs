pub mod main;

use std::error::Error;

pub trait Usecase: Send + Sync + 'static {
    fn health(&self) -> Result<(), Box<dyn Error>>;
}
