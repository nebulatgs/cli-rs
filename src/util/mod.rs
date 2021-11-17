pub mod client;
pub mod config;
pub mod consts;
pub mod spinner;

type UtilResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
