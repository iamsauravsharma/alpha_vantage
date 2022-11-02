//! Module which contains some common trait implementation for `Vec<Data>`

use crate::error::Result;

/// trait which helps for performing some common operation on `Vec<Data>` which
/// have given period
pub trait FindData: IntoIterator {
    /// Find a data with a given time as a input return none if no data found
    fn find(&self, time: &str) -> Option<&<Self as IntoIterator>::Item>;
    /// Return a data which is of latest time period
    fn latest(&self) -> <Self as IntoIterator>::Item;
    /// Return a top n latest Data
    /// # Errors
    /// If n is greater than no of data
    fn latest_n(&self, n: usize) -> Result<Vec<&<Self as IntoIterator>::Item>>;
}
