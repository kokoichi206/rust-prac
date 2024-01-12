use std::sync::Arc;

use crate::repository;
use crate::usecase;

pub struct UsecaseImpl {
    repository: Arc<dyn repository::Repository>,
}

impl UsecaseImpl {
    pub fn new(repo: Arc<dyn repository::Repository>) -> UsecaseImpl {
        UsecaseImpl { repository: repo }
    }
}

impl usecase::Usecase for UsecaseImpl {
    fn health(&self) -> Result<(), String> {
        self.repository.health()
    }
}
