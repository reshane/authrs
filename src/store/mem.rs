use super::{Store, error::{StoreResult, StoreError}};
use crate::types::{DataType, DataObject, User};
use std::{collections::HashMap, sync::RwLock};

#[derive(Debug)]
pub struct MemStore {
    users: RwLock<HashMap<i64, User>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::<i64, User>::new()),
        }
    }
}

impl Store for MemStore {
    async fn get(&self, id: i64, data_type: DataType) -> Option<Box<dyn DataObject>> {
        match data_type {
            DataType::User => {
                let users = self.users.read();
                match users {
                    Ok(users) => {
                        match users.get(&id) {
                            Some(data) => Some(Box::new(data.clone())),
                            None => None,
                        }
                    },
                    Err(_) => None,
                }
            },
        }
    }

    async fn create<'a>(&self, data: &'a dyn DataObject) -> StoreResult<&'a dyn DataObject> {
        match data.data_type() {
            DataType::User => {
                let downcasted = data.as_any().downcast_ref::<User>();
                if let Some(ins_data) = downcasted {
                    return match self.users.write() {
                        Ok(mut users) => {
                            users.insert(ins_data.id(), (*ins_data).clone());
                            Ok(data)
                        },
                        Err(_) => Err(StoreError::NotCreated),
                    };
                }
                Err(StoreError::NotCreated)
            },
        }
    }

    async fn delete(&self, id: i64, data_type: DataType) -> StoreResult<Box<dyn DataObject>> {
        match data_type {
            DataType::User => {
                match self.users.write() {
                    Ok(mut users) => {
                        match users.remove(&id) {
                            Some(data) => Ok(Box::new(data)),
                            None => Err(StoreError::NotFound),
                        }
                    },
                    Err(_) => Err(StoreError::NotFound),
                }
            },
        }
    }
}

