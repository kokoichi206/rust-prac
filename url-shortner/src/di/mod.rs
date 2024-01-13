use std::error::Error;
use std::sync::Arc;

use crate::repository::{self, sqlite};
// use crate::usecase::Usecase;
use crate::usecase;

// #[derive(Clone)]
// pub struct AppModule<R>
// where
//     R: repository::Repository,
// {
//     ucase: Arc<R>,
// }

#[derive(Clone)]
pub struct AppModule {
    ucase: Arc<dyn usecase::Usecase>,
}

// {
//     ucase: Arc<dyn Usecase>,
// }

impl AppModule {
    // impl<R> AppModule<R>
    // where
    //     R: repository::Repository,
    // {
    pub fn new() -> Result<AppModule, Box<dyn Error>> {
        // pub fn new() -> Result<AppModule<R>, Box<dyn Error>> {
        let database = sqlite::Database::new()?;
        let ucase = usecase::main::UsecaseImpl::new(Arc::new(database));

        Ok(AppModule {
            ucase: Arc::new(ucase),
        })
    }

    pub fn static_usecase(&self) -> &Arc<dyn usecase::Usecase> {
        // pub fn static_usecase(&self) -> &Arc<R> {
        &self.ucase
    }
}
