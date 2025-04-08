use super::{DataObject, RequestObject, ValidationError};
use serde::{Deserialize, Serialize};
use sqlite::{Bindable, BindableWithIndex, State};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub id: i64,
    pub owner_id: i64,
    pub contents: String,
}

impl Bindable for Note {
    fn bind(self, statement: &mut sqlite::Statement) -> sqlite::Result<()> {
        self.id.clone().bind(statement, 1)?;
        self.owner_id.clone().bind(statement, 2)?;
        self.contents.clone().as_str().bind(statement, 3)?;
        Ok(())
    }
}

impl DataObject for Note {
    fn from_rows(statement: &mut sqlite::Statement) -> Vec<Self> {
        let mut res = vec![];
        while let Ok(State::Row) = statement.next() {
            res.push(Self {
                id: statement.read::<i64, _>("id").unwrap(),
                owner_id: statement.read::<i64, _>("owner_id").unwrap(),
                contents: statement.read::<String, _>("contents").unwrap(),
            });
        }
        return res;
    }

    fn table_name() -> String { "notes".to_string() }

    fn sql_cols() -> String { "id,owner_id,contents".to_string() }

    fn id_col() -> String { "id".to_string() }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestNote {
    pub id: Option<i64>,
    pub owner_id: Option<i64>,
    pub contents: Option<String>,
}

impl Bindable for RequestNote {
    fn bind(self, statement: &mut sqlite::Statement) -> sqlite::Result<()> {
        let mut idx = 1;
        if let Some(id) = self.id {
            id.clone().bind(statement, idx)?;
            idx += 1;
        }
        if let Some(owner_id) = self.owner_id {
            owner_id.clone().bind(statement, idx)?;
            idx += 1;
        }
        if let Some(contents) = self.contents {
            contents.clone().as_str().bind(statement, idx)?;
        }
        Ok(())
    }
}

impl RequestObject for RequestNote {
    fn validate_create(&self) -> Result<(), ValidationError> {
        match self.owner_id {
            Some(_) => {},
            None => { return Err(ValidationError::MissingRequiredOnCreate(String::from("owner_id"))); },
        }
        match self.contents {
            Some(_) => {},
            None => { return Err(ValidationError::MissingRequiredOnCreate(String::from("contents"))); },
        }
        match self.id {
            Some(_) => { return Err(ValidationError::IdProvidedOnCreate); },
            None => {},
        }
        Ok(())
    }

    fn validate_update(&self) -> Result<(), ValidationError> {
        match self.id {
            Some(_) => Ok(()),
            None => Err(ValidationError::MissingIdOnUpdate),
        }
    }

    fn sql_cols(&self) -> String {
        let mut cols = vec![];
        if let Some(_) = self.id { cols.push("id"); }
        if let Some(_) = self.owner_id { cols.push("owner_id"); }
        if let Some(_) = self.contents { cols.push("contents"); }
        cols.join(",")
    }

    fn sql_placeholders(&self) -> String {
        let mut ct = 0;
        if let Some(_) = self.id { ct += 1; }
        if let Some(_) = self.owner_id { ct += 1; }
        if let Some(_) = self.contents { ct += 1; }
        vec!["?"; ct].join(",")
    }

    fn id(&self) -> Option<i64> {
        self.id
    }
}
