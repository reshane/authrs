use super::Store;
use sqlx::PgPool;

#[allow(dead_code)]
pub struct PsqlStore {
    pool: PgPool,
}

#[allow(dead_code)]
impl Store for PsqlStore {
    async fn get(&self, _id: i64, _data_type: super::DataType) -> Option<Box<dyn super::DataObject>> {
        todo!()
    }

    async fn create<'a>(&self, _data: &'a dyn super::DataObject) -> super::error::StoreResult<&'a dyn super::DataObject> {
        todo!()
    }

    async fn delete(&self, _id: i64, _data_type: super::DataType) -> super::error::StoreResult<Box<dyn super::DataObject>> {
        todo!()
    }
}
