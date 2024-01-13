pub mod sqlite;

use async_trait::async_trait;
use std::error::Error;
use std::future::Future;

// pub trait Repository: Send + Sync + 'static {
pub trait Repository: Clone + Send + Sync + 'static {
    // pub trait Repository: Send + Sync + 'static {
    // fn health(&self) -> Result<(), Box<dyn Error>>;
    // // async fn health(&self) -> Result<(),Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>;

    // fn search_url_from_short_url(&self, short_url: String) -> Result<String, Box<dyn Error>>;

    async fn health(&self) -> Result<(), Box<dyn Error>>;
    async fn search_url_from_short_url(&self, short_url: String) -> Result<String, Box<dyn Error>>;

    // fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>>;
    // fn search_url_from_short_url(
    //     &self,
    //     short_url: String,
    // ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>>;
}
