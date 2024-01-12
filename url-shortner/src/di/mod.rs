use crate::repository::sqlite;
use crate::usecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppModule {
    ucase: Arc<dyn usecase::Usecase>,
}

impl AppModule {
    pub fn new() -> AppModule {
        let database = sqlite::Database;
        let usecase = usecase::main::UsecaseImpl::new(Arc::new(database));

        AppModule {
            ucase: Arc::new(usecase),
        }
    }

    pub fn static_usecase(&self) -> &Arc<dyn usecase::Usecase> {
        &self.ucase
    }
}
