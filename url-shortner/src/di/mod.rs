use std::error::Error;
use std::sync::Arc;
use std::pin::Pin;

use crate::repository::{self, sqlite};
use crate::usecase;

use async_trait::async_trait;

// #[derive(Clone)]
// pub struct AppModule {
//     ucase: Pin<Arc<dyn usecase::Usecase>>,
// }

// impl AppModule {

//     pub fn new() -> Result<AppModule, Box<dyn Error>> {
//         // pub fn new() -> Result<AppModule<R>, Box<dyn Error>> {
//         let database = sqlite::Database::new()?;
//         let ucase = usecase::main::UsecaseImpl::new(Arc::new(database));

//         Ok(AppModule {
//             ucase: Arc::new(ucase),
//         })
//     }

//     pub fn static_usecase(&self) -> &Arc<dyn usecase::Usecase> {
//         // pub fn static_usecase(&self) -> &Arc<R> {
//         &self.ucase
//     }
// }
