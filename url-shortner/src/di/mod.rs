use std::error::Error;
use std::sync::Arc;

use crate::repository::sqlite;
use crate::usecase;

#[derive(Clone)]
pub struct AppModule {
    ucase: Arc<dyn usecase::Usecase>,
}

impl AppModule {
    pub async fn new() -> Result<AppModule, Box<dyn Error>> {
        let database = sqlite::Database::new().await?;
        let ucase = usecase::main::UsecaseImpl::new(Arc::new(database));

        Ok(AppModule {
            ucase: Arc::new(ucase),
        })
    }

    pub fn static_usecase(&self) -> &Arc<dyn usecase::Usecase> {
        &self.ucase
    }
}
