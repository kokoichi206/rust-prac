pub mod sqlite;

pub trait Repository: Send + Sync + 'static {
    fn health(&self) -> Result<(), String>;
}
