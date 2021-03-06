use async_trait::async_trait;

use crate::error::{Error, Result};

#[async_trait]
/// Trait which can be implemented for all common library client for getting
/// output from server
/// surf and reqwest are two client which are supported with feature flag. If
/// you prefer alternate http client you can add support by implementing
/// `HttpClient` trait for client.
/// Some example of other client which can be used are isahc client
pub trait HttpClient {
    /// Get output from server in String. This string is automatically converted
    /// to appropriate struct by library
    async fn get_output(&self, path: String) -> Result<String>;
}

#[cfg(feature = "reqwest-client")]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn get_output(&self, path: String) -> Result<String> {
        self.get(&path)
            .send()
            .await
            .map_err(|_| Error::GetRequestFailed)?
            .text()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }
}

#[cfg(feature = "surf-client")]
#[async_trait]
impl HttpClient for surf::Client {
    async fn get_output(&self, path: String) -> Result<String> {
        self.get(path)
            .recv_string()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }
}
