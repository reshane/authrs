use crate::types::{DataObject, DataType};

pub(crate) mod error;

mod psql;
pub use psql::PsqlStore;

pub(crate) trait Store: Send {}
