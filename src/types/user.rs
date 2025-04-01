use super::{DataObject, DataType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub picture: String,
}

impl DataObject for User {
    fn id(&self) -> i64 {
        self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data_type(&self) -> super::DataType {
        DataType::User
    }
}
