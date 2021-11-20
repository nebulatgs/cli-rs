use self::errors::RailwayError;

pub mod client;
pub mod config;
pub mod consts;
pub mod errors;

type UtilResult<T> = Result<T, RailwayError>;
