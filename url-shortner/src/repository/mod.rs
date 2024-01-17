pub mod sqlite;
pub mod error;

use async_trait::async_trait;
use std::error::Error;

// pub trait Repository: Send + Sync + 'static {
// pub trait Repository: Clone + Send + Sync + 'static {
#[async_trait]
pub trait Repository: Clone + Send + Sync + 'static {
    // pub trait Repository: Send + Sync + 'static {
    // fn health(&self) -> Result<(), Box<dyn Error>>;
    // // async fn health(&self) -> Result<(),Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>;

    // fn search_url_from_short_url(&self, short_url: String) -> Result<String, Box<dyn Error>>;

    async fn health(&self) -> anyhow::Result<()>;
    async fn search_url_from_short_url(&self, short_url: String) -> anyhow::Result<String>;

    // fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>>;
    // fn search_url_from_short_url(
    //     &self,
    //     short_url: String,
    // ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>>;
}
