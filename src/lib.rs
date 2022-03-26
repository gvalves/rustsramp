use std::{fmt::Display, ops::Deref};

pub mod app;
pub mod cli;
pub mod sequence;
pub mod utils;

pub use app::run;
pub use cli::Cli;

pub type Result<T = (), E = Box<dyn std::error::Error>> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    value: String,
}

impl Error {
    pub fn new(value: &str) -> Self {
        let value = String::from(value);
        Self { value }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::error::Error for Error {}

impl Deref for Error {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
