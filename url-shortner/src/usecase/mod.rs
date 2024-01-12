pub mod main;

pub trait Usecase: Send + Sync + 'static {
    fn health(&self) -> Result<(), String>;
}
