pub(crate) mod error;

mod psql;
use std::collections::HashMap;

use error::StoreResult;
pub use psql::PsqlStore;

use crate::types::DataObject;

pub(crate) trait Store: Send {}

pub(crate) trait Storeable<T, DO>
where
    T: Store,
    DO: DataObject,
{
    async fn get(id: i64, store: &T) -> Option<DO>;
    async fn get_queries(queries: &HashMap<String, String>, store: &T) -> Vec<DO>;
    async fn create(&self, store: &T) -> StoreResult<DO>;
    async fn delete(id: i64, store: &T) -> StoreResult<DO>;
}
