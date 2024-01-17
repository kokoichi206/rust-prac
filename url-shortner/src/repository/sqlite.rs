use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

use crate::domain::apperr::AppError;

#[derive(Clone)]
pub struct Database {
    client: Arc<Mutex<Client>>,
}

impl Database {
    pub async fn new() -> Result<Database, Box<dyn Error>> {
        // TODO: Read from config or env.
        let connection_string = format!(
            "host={} port={} user={} password={} dbname={}",
            "localhost", "6666", "root", "root", "postgresql"
        );

        let (client, conn) = tokio_postgres::connect(&connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        return Ok(Database {
            client: Arc::new(Mutex::new(client)),
        });
    }
}

const SEARCH_URL_FROM_SHORT_URL_STMT: &str = "
SELECT
    url
FROM shorturl
WHERE short = $1;
";

// 実装側にも async_trait を付ける。
#[async_trait]
impl super::Repository for Database {
    async fn health(&self) -> anyhow::Result<()> {
        let client = self.client.lock().await;
        let result = Box::new(client.query("SELECT 1", &[]).await?);

        if let Some(row) = result.get(0) {
            let value: i32 = row.get(0);
            if value == 1 {
                Ok(())
            } else {
                Err(AppError::Unexpected(format!("Expected 1, got {}", value)).into())
            }
        } else {
            Err(AppError::Unexpected(format!("Expected 1 row, got 0 rows")).into())
        }
    }

    async fn search_url_from_short_url(&self, short_url: String) -> anyhow::Result<String> {
        let client = self.client.lock().await;
        let rows = Box::new(
            client
                .query(SEARCH_URL_FROM_SHORT_URL_STMT, &[&short_url])
                .await?,
        );

        if rows.len() == 1 {
            if let Some(row) = rows.get(0) {
                let url: String = row.get(0);
                return Ok(url);
            }
            Err(AppError::NotFound(short_url).into())
        } else {
            Err(AppError::Unexpected(format!("Expected 1 row, got {} rows", rows.len())).into())
        }
    }
}
