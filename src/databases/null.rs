use crate::{AxumDatabasePool, SessionError};
use async_trait::async_trait;

/// Null Pool type for AxumDatabasePool.
/// Use this when you do not want to load any database.
pub struct AxumNullPool;

#[async_trait]
impl AxumDatabasePool for AxumNullPool {
    async fn migrate(&self, _table_name: &str) -> Result<(), SessionError> {
        Ok(())
    }

    async fn delete_by_expiry(&self, _table_name: &str) -> Result<(), SessionError> {
        Ok(())
    }

    async fn count(&self, _table_name: &str) -> Result<i64, SessionError> {
        return Ok(0);
    }

    async fn store(
        &self,
        _id: &str,
        _session: &str,
        _expires: i64,
        _table_name: &str,
    ) -> Result<(), SessionError> {
        Ok(())
    }

    async fn load(&self, _id: &str, _table_name: &str) -> Result<Option<String>, SessionError> {
        Ok(None)
    }

    async fn delete_one_by_id(&self, _id: &str, _table_name: &str) -> Result<(), SessionError> {
        Ok(())
    }

    async fn delete_all(&self, _table_name: &str) -> Result<(), SessionError> {
        Ok(())
    }
}
