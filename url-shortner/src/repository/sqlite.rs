use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::thread;
// use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};
// use postgres::{Client, NoTls};
use std::sync::Mutex;

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

impl super::Repository for Database {
    // async fn health(&self) -> Result<(), Box<dyn Future<Output = Result<(), Box<dyn Error>>>>> {
    //     let client = self.client.lock().unwrap();

    //     let result = client.query("SELECT 1", &[]).await;

    //     if let Some(row) = result.get(0) {
    //         let value: i32 = row.get(0);
    //         if value == 1 {
    //             Ok(())
    //         } else {
    //             Err("Unexpected result from database".into())
    //         }
    //     } else {
    //         Err("No rows returned from database".into())
    //     }

    //     // rt.block_on(async {
    //     //     let result = client.query("SELECT 1", &[]).await?;

    //     //     if let Some(row) = result.get(0) {
    //     //         let value: i32 = row.get(0);
    //     //         if value == 1 {
    //     //             Ok(())
    //     //         } else {
    //     //             Err("Unexpected result from database".into())
    //     //         }
    //     //     } else {
    //     //         Err("No rows returned from database".into())
    //     //     }
    //     // })
    // }
    // fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>> {
    // fn health(&self) -> Result<(), Box<dyn Error>> {

    // fn health(&self) -> impl Sized + Send + Future<Output = Result<(), Box<dyn Error>>> {
    async fn health(&self) -> Result<(), Box<dyn Error>> {
        // async fn health(&self) -> Result<(), Box<dyn Error>> {
        // let client = self.client.lock().unwrap();

        // let rt = tokio::runtime::Runtime::new().unwrap();
        // let result = thread::spawn(move || client.query("SELECT 1", &[]).unwrap())
        //     .join()
        //     .unwrap();

        // if let Some(row) = result.get(0) {
        //     let value: i32 = row.get(0);
        //     if value == 1 {
        //         Ok(())
        //     } else {
        //         Err("Unexpected result from database".into())
        //     }
        // } else {
        //     Err("No rows returned from database".into())
        // }

        // a =============================
        // let result = client.query("SELECT 1", &[]).await?;

        // if let Some(row) = result.get(0) {
        //     let value: i32 = row.get(0);
        //     if value == 1 {
        //         Ok(())
        //     } else {
        //         Err("Unexpected result from database".into())
        //     }
        // } else {
        //     Err("No rows returned from database".into())
        // }

        // let mut client = self.client.lock().unwrap();
        // let result = client.query("SELECT 1", &[])?;

        // let result = client.query("SELECT 1", &[]).await?;

        async {
            let client = self.client.lock().unwrap();
            let result = Box::new(client.query("SELECT 1", &[]).await?);

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
        }
        .await

        // rt.block_on(async {
        //     // let lock = self.client.lock();
        //     // let client = match lock {
        //     //     Ok(client) => client,
        //     //     Err(_) => return Err("Failed to lock client".into()),
        //     // };

        //     let result = client.query("SELECT 1", &[]).await?;

        //     if let Some(row) = result.get(0) {
        //         let value: i32 = row.get(0);
        //         if value == 1 {
        //             Ok(())
        //         } else {
        //             Err("Unexpected result from database".into())
        //         }
        //     } else {
        //         Err("No rows returned from database".into())
        //     }
        // })
    }

    async fn search_url_from_short_url(
        &self,
        short_url: String,
        // ) -> impl Sized + Send + Future<Output = Result<String, Box<dyn Error>>> {
    ) -> Result<String, Box<dyn Error>> {
        // async fn search_url_from_short_url(&self, short_url: String) -> Result<String, Box<dyn Error>> {
        // let client = self.client.clone();

        // let client = self.client.lock().unwrap();

        // let client = self.client.lock().await;

        // let mut client = self.client.lock().unwrap();
        // // let mut client = self.client.clone();

        // let rows = client.query(SEARCH_URL_FROM_SHORT_URL_STMT, &[&short_url])?;

        // let rows = client.query(SEARCH_URL_FROM_SHORT_URL_STMT, &[&short_url]).await?;

        // let su = &short_url;
        async move {
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
                Err("Failed to get single row".into())
            } else {
                Err("Unexpected row counts".into())
            }
        }
        .await

        // let rt = tokio::runtime::Runtime::new().unwrap();
        // rt.block_on(async {
        //     // let lock = self.client.lock();
        //     // let client = match lock {
        //     //     Ok(client) => client,
        //     //     Err(_) => return Err("Failed to lock client".into()),
        //     // };

        //     let client = self.client.lock().await;

        //     let rows = client
        //         .query(SEARCH_URL_FROM_SHORT_URL_STMT, &[&short_url])
        //         .await?;

        //     if rows.len() == 1 {
        //         if let Some(row) = rows.get(0) {
        //             let url: String = row.get(0);
        //             return Ok(url);
        //         }
        //         Err("Failed to get single row".into())
        //     } else {
        //         Err("Unexpected row counts".into())
        //     }
        // })

        // let rows = client
        //     .query(SEARCH_URL_FROM_SHORT_URL_STMT, &[&short_url])
        //     .await?;

        // if rows.len() == 1 {
        //     if let Some(row) = rows.get(0) {
        //         let url: String = row.get(0);
        //         return Ok(url);
        //     }
        //     Err("Failed to get single row".into())
        // } else {
        //     Err("Unexpected row counts".into())
        // }
    }
}
