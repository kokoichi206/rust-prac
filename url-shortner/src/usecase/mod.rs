pub mod main;

use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Usecase: Clone + Send + Sync + 'static {
    async fn health(&self) -> anyhow::Result<()>;
    async fn search_original_url(&self, short_url: String) -> anyhow::Result<String>;
}
