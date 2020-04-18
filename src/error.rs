//! Module which contains all types of error for alpha vantage crates
use thiserror::Error;

/// Result type for alpha vantage crate
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Main error/failure enum
pub enum Error {
    /// Error which is raised if information is returned instead of data from
    /// API
    #[error("information: {0}")]
    AlphaVantageInformation(String),

    /// Error which is raised if error_message is raised instead of data from
    /// API
    #[error("error: {0}")]
    AlphaVantageErrorMessage(String),

    /// Error which is raised when desired number of entry is not present
    #[error("desired number of latest entry not found try using less than {0} as n")]
    DesiredNumberOfEntryNotPresent(usize),
}
