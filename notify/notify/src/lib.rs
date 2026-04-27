pub mod config;
mod err;
pub mod handler;
pub mod init;
mod state;

pub use err::*;
pub use state::*;
pub type Result<T> = std::result::Result<T, crate::Error>;
