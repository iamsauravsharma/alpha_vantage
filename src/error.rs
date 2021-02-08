//! Module which contains all types of error for alpha vantage crates
use thiserror::Error;

/// Result type for alpha vantage crate
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Main error/failure enum
pub enum Error {
    /// Error which is raised if information is returned by API instead of data
    /// from API
    #[error("information: {0}")]
    AlphaVantageInformation(String),

    /// Error which is raised if error_message is returned by API instead of
    /// data from API
    #[error("error_message: {0}")]
    AlphaVantageErrorMessage(String),

    /// Error which is raised if note is returned by API instead of data from
    /// API
    #[error("note: {0}")]
    AlphaVantageNote(String),

    /// Error which is raised when desired number of entry is not present
    #[error("desired number of latest entry not found try using less than {0} as n")]
    DesiredNumberOfEntryNotPresent(usize),

    /// Error which is raised if API return empty response instead of returning
    /// data
    #[error("server returned empty response")]
    EmptyResponse,

    /// Error which is raised if client fails to get json from server and decode
    /// it into struct
    #[error("Failed to get json and decode into struct")]
    DecodeJsonToStruct,
}
