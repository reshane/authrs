use std::collections::HashMap;

use crate::store::{
    PsqlStore, Storeable,
    error::{StoreError, StoreResult},
};

use super::{DataMeta, DataObject};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::{debug, error};

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub guid: String,
    pub name: String,
    pub email: String,
    pub picture: String,
}

impl DataObject for User {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_owner_author_id(&self) -> i32 {
        self.id
    }
}
impl DataMeta for User {
    fn get_id_col() -> &'static str {
        "id"
    }
    fn get_owner_author_col() -> &'static str {
        "id"
    }
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

    async fn get_queries(queries: &HashMap<String, String>, store: &PsqlStore) -> Vec<User> {
        let mut clauses = vec![];
        let mut values = vec![];
        for (i, (k, v)) in queries.iter().enumerate() {
            debug!("{} = ${} ({})", k, i + 1, v);
            clauses.push(format!("({} = ${})", k, i + 1));
            values.push(v);
        }
        let sql = if clauses.len() == 0 {
            "select * from users".to_string()
        } else {
            format!("select * from users where {}", clauses.join(" and "))
        };

        debug!("{}", sql);

        let mut query = sqlx::query_as::<_, User>(sql.as_str());

        for v in values.into_iter() {
            query = query.bind(v);
        }

        let users = query.fetch_all(&store.pool).await;
        match users {
            Ok(users) => users,
            Err(e) => {
                error!("{:?}", e);
                vec![]
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
