pub(crate) mod sqlitestore;
pub(crate) mod error;
pub use sqlitestore::SqliteStore;

use error::StoreResult;
use sqlite::Value;

use crate::types::{DataObject, RequestObject};

pub(crate) trait Store {
    fn create<R: RequestObject, T: DataObject>(&self, data: R) -> StoreResult<T>;
    fn update<R: RequestObject, T: DataObject>(&self, data: R) -> StoreResult<T>;
    fn get<T: DataObject>(&self, id: i64) -> Option<T>;
    fn get_queries<T: DataObject>(&self, queries: Vec<Box<dyn Query>>) -> Vec<T>;
    fn delete<T: DataObject>(&self, id: i64) -> StoreResult<T>;
}

pub(crate) trait Query {
    fn build(&self) -> (String, Value);
}

#[allow(dead_code)]
pub(crate) struct ContainsQuery {
    pub field: String,
    pub val: String,
}

impl Query for ContainsQuery {
    fn build(&self) -> (String, Value) {
        (
            format!("{} LIKE ?", self.field),
            Value::String(format!("%{}%", self.val)),
        )
    }
}

pub(crate) struct EqualsQuery {
    pub field: String,
    pub val: Value,
}

impl Query for EqualsQuery {
    fn build(&self) -> (String, Value) {
        (format!("{} = ?", self.field), self.val.clone())
    }
}
