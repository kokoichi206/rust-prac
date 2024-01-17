use std::{error::Error, sync::Arc};

use crate::repository;

use async_trait::async_trait;

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
    pub fn new(repo: Arc<R>) -> UsecaseImpl<R> {
        UsecaseImpl { repository: repo }
    }
}

#[async_trait]
impl<R> super::Usecase for UsecaseImpl<R>
where
    R: repository::Repository,
{
    async fn health(&self) -> anyhow::Result<()> {
        self.repository.health().await
    }

    async fn search_original_url(&self, short_url: String) -> anyhow::Result<String> {
        self.repository.search_url_from_short_url(short_url).await
    }
}
