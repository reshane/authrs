use super::DataObject;
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
}
