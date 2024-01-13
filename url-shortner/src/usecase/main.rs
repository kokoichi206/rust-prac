use std::{error::Error, sync::Arc};

use crate::repository;

// こいつに対して Clone を定義するには？
#[derive(Clone)]
pub struct UsecaseImpl<R>
where
    R: repository::Repository,
{
    repository: Arc<R>,
}

impl<R> UsecaseImpl<R>
where
    R: repository::Repository,
{
    // <T: Repository>(repo: &T)
    // pub fn new(repo: Arc<dyn repository::Repository>) -> UsecaseImpl {
    //     UsecaseImpl { repository: repo }
    // }
    pub fn new(repo: Arc<R>) -> Self {
        UsecaseImpl { repository: repo }
    }
}

// pub struct UsecaseImpl {
//     repository: Arc<dyn repository::Repository>,
// }

// impl UsecaseImpl {
//     <T: Repository>(repo: &T)
//     pub fn new(repo: Arc<dyn repository::Repository>) -> UsecaseImpl {
//         UsecaseImpl { repository: repo }
//     }
//     pub fn new(repo: Arc<dyn repository::Repository>) -> Self {
//         UsecaseImpl { repository: repo }
//     }
// }

// impl super::Usecase for UsecaseImpl {
//     fn health(&self) -> Result<(), Box<dyn Error>> {
//         self.repository.health()
//     }

//     fn search_original_url(&self, short_url: String) -> Result<String, Box<dyn Error>> {
//         self.repository.search_url_from_short_url(short_url)
//     }
// }

use std::future::Future;

impl<R> super::Usecase for UsecaseImpl<R>
where
    R: repository::Repository,
{
    async fn health(&self) -> Result<(), Box<dyn Error>> {
    // fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>> {
    // fn health(&self) -> Result<(), Box<dyn Error>> {
        // async { self.repository.health().await? }.await
        self.repository.health().await
    }

    async fn search_original_url(&self, short_url: String) -> Result<String, Box<dyn Error>> {
    // fn search_original_url(
    //     &self,
    //     short_url: String,
    //     // ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>> {
    // ) -> Result<String, Box<dyn Error>> {
        // async { self.repository.search_url_from_short_url(short_url).await? }.await
        self.repository.search_url_from_short_url(short_url).await
    }
}
