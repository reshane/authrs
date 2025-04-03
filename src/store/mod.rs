use crate::types::{DataObject, DataType};

mod error;
use error::StoreResult;

mod mem;
pub use mem::MemStore;

mod psql;

pub(crate) trait Store: Send {
    async fn get(&self, id: i64, data_type: DataType) -> Option<Box<dyn DataObject>>;
    async fn create<'a>(&self, data: &'a dyn DataObject) -> StoreResult<&'a dyn DataObject>;
    async fn delete(&self, id: i64, data_type: DataType) -> StoreResult<Box<dyn DataObject>>;
}

