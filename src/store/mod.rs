use crate::types::DataObject;

pub(crate) trait Store<T> {
    fn get(&self, id: i64) -> Option<&T>
    where
        T: DataObject;
    fn create(&mut self, data: T) -> StoreResult<&T>
    where
        T: DataObject;
    fn delete(&mut self, id: i64) -> Option<T>
    where
        T: DataObject;
}

// in memory storage
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemStore<T>
where
    T: DataObject,
{
    mem: HashMap<i64, T>,
}

impl<T> MemStore<T>
where
    T: DataObject
{
    pub fn new() -> Self {
        Self {
            mem: HashMap::<i64, T>::new()
        }
    }
}

impl<T: DataObject> Store<T> for MemStore<T> {
    fn get(&self, id: i64) -> Option<&T>
    where
        T: DataObject,
    {
        self.mem.get(&id)
    }

    fn create(&mut self, data: T) -> StoreResult<&T>
    where
        T: DataObject,
    {
        self.mem.insert(data.id(), data.clone());
        match self.mem.get(&data.id()) {
            Some(d) => Ok(d),
            None => Err(StoreError::NotCreated),
        }
    }

    fn delete(&mut self, id: i64) -> Option<T>
    where
        T: DataObject,
    {
        self.mem.remove(&id)
    }
}

use std::error::Error;
use std::fmt;

// Store error kinds
#[derive(Debug)]
pub enum StoreError {
    NotCreated,
}

impl fmt::Display for StoreError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            StoreError::NotCreated => {
                write!(fmt, "The data could not be created",)
            }
        }
    }
}

impl Error for StoreError {
    fn description(&self) -> &str {
        match *self {
            StoreError::NotCreated => "NotCreated error",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            StoreError::NotCreated => None,
        }
    }
}

// impl From<io::Error> for Store {
// fn from(err: io::Error) -> WbmpError {
// WbmpError::IoError(err)
// }
// }

// Result of a Store operation
pub type StoreResult<T> = Result<T, StoreError>;
