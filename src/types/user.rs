use crate::{
    Store,
    store::PsqlStore,
    store::error::{StoreError, StoreResult},
};

use super::{DataObject, DataType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::error;

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub guid: String,
    pub name: String,
    pub email: String,
    pub picture: String,
}

impl DataObject for User {
    fn id(&self) -> i32 {
        self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data_type(&self) -> super::DataType {
        DataType::User
    }
}

pub(crate) trait Storeable<T, DO>
where
    T: Store,
    DO: DataObject,
{
    async fn get(id: i64, store: &T) -> Option<DO>;
    async fn create(&self, store: &T) -> StoreResult<DO>;
    async fn delete(id: i64, store: &T) -> StoreResult<DO>;
}

impl Storeable<PsqlStore, User> for User {
    async fn get(id: i64, store: &PsqlStore) -> Option<User> {
        let user = sqlx::query_as::<_, User>("select * from users where id = ($1)")
            .bind(id)
            .fetch_one(&store.pool)
            .await;
        match user {
            Ok(user) => Some(user),
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }

    async fn create(&self, store: &PsqlStore) -> StoreResult<User> {
        let user = sqlx::query_as::<_, User>(
            "insert into users (guid,name,email,picture) values ($1,$2,$3,$4) returning *",
        )
        .bind(self.guid.clone())
        .bind(self.name.clone())
        .bind(self.email.clone())
        .bind(self.picture.clone())
        .fetch_one(&store.pool)
        .await;
        match user {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("{:?}", e);
                Err(StoreError::NotCreated)
            }
        }
    }

    async fn delete(id: i64, store: &PsqlStore) -> StoreResult<User> {
        let user = sqlx::query_as::<_, User>("delete from users where id = ($1) returning *")
            .bind(id)
            .fetch_one(&store.pool)
            .await;
        match user {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("{:?}", e);
                Err(StoreError::NotFound)
            }
        }
    }
}
