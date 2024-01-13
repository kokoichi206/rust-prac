use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, NoTls};

pub struct Database {
    client: Arc<Mutex<Client>>,
}

impl Database {
    pub fn new() -> Result<Database, Box<dyn Error>> {
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
            });
        })
    }
}

impl super::Repository for Database {
    fn health(&self) -> Result<(), Box<dyn Error>> {
        let client = self.client.lock().unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let result = client.query("SELECT 1", &[]).await?;

            if let Some(row) = result.get(0) {
                let value: i32 = row.get(0);
                if value == 1 {
                    Ok(())
                } else {
                    Err("Unexpected result from database".into())
                }
            } else {
                Err("No rows returned from database".into())
            }
        })
    }
}
