use std::{error::Error, sync::Arc};

use crate::repository;

pub struct UsecaseImpl {
    repository: Arc<dyn repository::Repository>,
}

impl UsecaseImpl {
    pub fn new(repo: Arc<dyn repository::Repository>) -> UsecaseImpl {
        UsecaseImpl { repository: repo }
    }
}

impl super::Usecase for UsecaseImpl {
    fn health(&self) -> Result<(), Box<dyn Error>> {
        self.repository.health()
    }
}
