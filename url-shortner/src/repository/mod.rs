pub mod sqlite;

use async_trait::async_trait;

#[async_trait]
pub trait Repository: Clone + Send + Sync + 'static {
    async fn health(&self) -> anyhow::Result<()>;
    async fn search_url_from_short_url(&self, short_url: String) -> anyhow::Result<String>;
}
