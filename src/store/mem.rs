use super::{Store, error::{StoreResult, StoreError}};
use crate::types::{DataType, DataObject, User};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemStore {
    users: HashMap<i64, User>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::<i64, User>::new(),
        }
    }
}

impl Store for MemStore {
    fn get(&self, id: i64, data_type: DataType) -> Option<&dyn DataObject> {
        let data = match data_type {
            DataType::User => self.users.get(&id)
        };

        match data {
            Some(data) => Some(data),
            None => None,
        }
    }

    fn create<'a>(&mut self, data: &'a dyn DataObject) -> StoreResult<&'a dyn DataObject> {
        match data.data_type() {
            DataType::User => {
                let downcasted = data.as_any().downcast_ref::<User>();
                if let Some(ins_data) = downcasted {
                    self.users.insert(ins_data.id(), (*ins_data).clone());
                    return Ok(data);
                }
                Err(StoreError::NotCreated)
            },
        }
    }

    fn delete(&mut self, id: i64, data_type: DataType) -> StoreResult<Box<dyn DataObject>> {
        let data = match data_type {
            DataType::User => self.users.remove(&id),
        };

        match data {
            Some(data) => Ok(Box::new(data)),
            None => Err(StoreError::NotFound),
        }
    }
}

