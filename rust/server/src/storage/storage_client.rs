use async_trait::async_trait;
use axum::http::Uri;
use tokio::io::AsyncWriteExt;

fn is_file_uri(uri: &Uri) -> bool {
    // if scheme is none
    uri.scheme().is_none()
}

#[async_trait]
pub trait StorageClient {
    // Get a json value from the storage
    async fn get(&self, key: &str) -> Option<serde_json::Value>;
    // Set a json value in the storage; will create new file if it doesnt exist or overwrite otherwise
    async fn set(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()>;
    // Delete from the storage; returns true if the value was deleted
    async fn delete(&self, key: &str) -> bool;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_uri() {

        let uri = Uri::from_static("http://example.com");
        assert!(!is_file_uri(&uri));

        let uri = Uri::from_static("/tmp");
        assert!(is_file_uri(&uri));

        let uri = Uri::from_static("tmp");
        assert!(is_file_uri(&uri));
    }

}