use crate::usecase;

use std::error::Error;
use std::sync::Arc;

use super::repository::sqlite;
use super::usecase::Usecase;

#[derive(Clone)]
pub struct AppModule {
    ucase: Arc<dyn Usecase>,
}

impl AppModule {
    pub fn new() -> Result<AppModule, Box<dyn Error>> {
        let database = sqlite::Database::new()?;
        let ucase = usecase::main::UsecaseImpl::new(Arc::new(database));

        Ok(AppModule {
            ucase: Arc::new(ucase),
        })
    }

    pub fn static_usecase(&self) -> &Arc<dyn Usecase> {
        &self.ucase
    }
}
