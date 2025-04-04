use async_trait::async_trait;

#[async_trait]
pub trait StorageClient: Send + Clone {
    // Get a json value from the storage
    async fn get(&self, key: &str) -> Option<serde_json::Value>;
    // Set a json value in the storage; will create new file if it doesnt exist or overwrite otherwise
    async fn set(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()>;
    // Delete from the storage; returns true if the value was deleted
    async fn delete(&self, key: &str) -> bool;
}

#[derive(Debug, Clone)]
pub struct NotImplementedYetClient {}

#[async_trait]
impl StorageClient for NotImplementedYetClient {
    async fn get(&self, _key: &str) -> Option<serde_json::Value> {
        None
    }

    async fn set(&self, _key: &str, _value: serde_json::Value) -> anyhow::Result<()> {
        Ok(())
    }

    async fn delete(&self, _key: &str) -> bool {
        false
    }
}