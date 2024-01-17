pub mod main;

use async_trait::async_trait;

#[async_trait]
pub trait Usecase: Send + Sync {
    async fn health(&self) -> anyhow::Result<()>;
    async fn search_original_url(&self, short_url: String) -> anyhow::Result<String>;
}
