pub mod main;

use std::error::Error;

use async_trait::async_trait;

use std::future::Future;

pub trait Usecase: Send + Sync + 'static {
    // fn health(&self) -> Result<(), Box<dyn Error>>;
    // fn search_original_url(&self, short_url: String) -> Result<String, Box<dyn Error>>;

    // async fn health(&self) -> Result<(), Box<dyn Error>>;
    // async fn search_original_url(&self, short_url: String) -> Result<String, Box<dyn Error>>;

    // fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>>;
    // fn search_url_from_short_url(
    //     &self,
    //     short_url: String,
    // ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>>;
    fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>>;
    fn search_original_url(
        &self,
        short_url: String,
    ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>>;

    // fn search_url_from_short_url(
    //     &self,
    //     short_url: String,
    // ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>>;
}

// #[async_trait]
// pub trait Usecase: Send + Sync + 'static {
//     async fn health(&self) -> Result<(), Box<dyn Error>>;
//     async fn search_original_url(&self, short_url: String) -> Result<String, Box<dyn Error>>;
// }
