pub struct Database;

use crate::repository;

impl Database {}

impl repository::Repository for Database {
    fn health(&self) -> Result<(), String> {
        Ok(())
    }
}
