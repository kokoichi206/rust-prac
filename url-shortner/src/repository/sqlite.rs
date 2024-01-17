use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::thread;
// use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};
// use postgres::{Client, NoTls};
use std::sync::Mutex;

use crate::repository::error::RepositoryError;

use async_trait::async_trait;

#[derive(Clone)]
pub struct Database {
    client: Arc<Mutex<Client>>,
    // client: Arc<Client>,
}

impl Database {
    pub fn new() -> Result<Database, Box<dyn Error>> {
        // TODO: Read from config or env.
        let connection_string = format!(
            "host={} port={} user={} password={} dbname={}",
            "localhost", "6666", "root", "root", "postgresql"
        );

        // let (client, _) = tokio_postgres::connect(&connection_string, NoTls).await?;
        // // let client = Client::connect(&connection_string, NoTls)?;
        // return Ok(Database {
        //     client: Arc::new(Mutex::new(client)),
        //     // client: Arc::new(client),
        // });

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // TODO: Read from config or env.
            let connection_string = format!(
                "host={} port={} user={} password={} dbname={}",
                "localhost", "6666", "root", "root", "postgresql"
            );

            let (client, _) = tokio_postgres::connect(&connection_string, NoTls).await?;
            return Ok(Database {
                client: Arc::new(Mutex::new(client)),
                // client: Arc::new(client),
            });
        })
    }
}

const SEARCH_URL_FROM_SHORT_URL_STMT: &str = "
SELECT
    url
FROM shorturl
WHERE short = $1;
";

// 実装側に async_trait を付ける。
#[async_trait]
impl super::Repository for Database {
    async fn health(&self) -> anyhow::Result<()> {
        let client = self.client.lock().unwrap();
        let result = Box::new(client.query("SELECT 1", &[]).await?);

        if let Some(row) = result.get(0) {
            let value: i32 = row.get(0);
            if value == 1 {
                Ok(())
            } else {
                RepositoryError::Unexpected(format!("Expected 1, got {}", value))
            }
        } else {
            RepositoryError::Unexpected("Expected 1 row, got 0 rows".into())
        }
    }

    async fn search_url_from_short_url(&self, short_url: String) -> anyhow::Result<String> {
        let client = self.client.lock().unwrap();
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
            RepositoryError::NotFound(short_url)
        } else {
            RepositoryError::Unexpected(format!("Expected 1 row, got {} rows", rows.len()))
        }
    }
}
