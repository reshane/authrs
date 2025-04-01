use crate::types::{DataObject, DataType};

mod error;
use error::StoreResult;

mod mem;
pub use mem::MemStore;

pub(crate) trait Store {
    fn get(&self, id: i64, data_type: DataType) -> Option<&dyn DataObject>;
    fn create<'a>(&mut self, data: &'a dyn DataObject) -> StoreResult<&'a dyn DataObject>;
    fn delete(&mut self, id: i64, data_type: DataType) -> StoreResult<Box<dyn DataObject>>;
}

