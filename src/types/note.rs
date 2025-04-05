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
pub struct Note {
    pub id: i32,
    pub owner_id: i32,
    pub contents: String,
}

impl DataObject for Note {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_owner_author_id(&self) -> i32 {
        self.owner_id
    }
}

impl DataMeta for Note {
    fn get_id_col() -> &'static str {
        "id"
    }
    fn get_owner_author_col() -> &'static str {
        "owner_id"
    }
}

impl Storeable<PsqlStore, Note> for Note {
    async fn get(id: i64, store: &PsqlStore) -> Option<Note> {
        let note = sqlx::query_as::<_, Note>("select * from notes where id = ($1)")
            .bind(id)
            .fetch_one(&store.pool)
            .await;
        match note {
            Ok(note) => Some(note),
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }

    async fn get_queries(queries: &HashMap<String, String>, store: &PsqlStore) -> Vec<Note> {
        let mut clauses = vec![];
        let mut values = vec![];
        for (i, (k, v)) in queries.iter().enumerate() {
            debug!("{} = ${} ({})", k, i + 1, v);
            clauses.push(format!("({} = ${})", k, i + 1));
            values.push(v);
        }
        let sql = if clauses.len() == 0 {
            "select * from notes".to_string()
        } else {
            format!("select * from notes where {}", clauses.join(" and "))
        };

        debug!("{}", sql);

        let mut query = sqlx::query_as::<_, Note>(sql.as_str());

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

    async fn create(&self, store: &PsqlStore) -> StoreResult<Note> {
        let note = sqlx::query_as::<_, Note>(
            "insert into notes (owner_id,contents) values ($1,$2) returning *",
        )
        .bind(self.owner_id.clone())
        .bind(self.contents.clone())
        .fetch_one(&store.pool)
        .await;
        match note {
            Ok(note) => Ok(note),
            Err(e) => {
                error!("{:?}", e);
                Err(StoreError::NotCreated)
            }
        }
    }

    async fn delete(id: i64, store: &PsqlStore) -> StoreResult<Note> {
        let note = sqlx::query_as::<_, Note>("delete from notes where id = ($1) returning *")
            .bind(id)
            .fetch_one(&store.pool)
            .await;
        match note {
            Ok(note) => Ok(note),
            Err(e) => {
                error!("{:?}", e);
                Err(StoreError::NotFound)
            }
        }
    }
}
