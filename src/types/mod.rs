use std::fmt;

use serde::Deserialize;

mod user;
use sqlite::{Bindable, Statement};
pub use user::{User, RequestUser};
mod note;
pub use note::{Note, RequestNote};

pub trait DataObject: Sized + Bindable + std::fmt::Debug + Clone {
    fn from_rows(statement: &mut Statement) -> Vec<Self>;
    fn table_name() -> String;
    fn sql_cols() -> String;
    fn id_col() -> String;
}

pub trait DataVisitor {
    fn visit_user(&self, u: &User);
    fn visit_note(&self, n: &Note);
}

#[derive(Debug, Deserialize)]
pub(crate) enum DataType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "note")]
    Note,
}

pub(crate) trait RequestObject: Bindable {
    fn validate_create(&self) -> Result<(), ValidationError>;
    fn validate_update(&self) -> Result<(), ValidationError>;
    fn sql_cols(&self) -> String;
    fn sql_placeholders(&self) -> String;
    fn id(&self) -> Option<i64>;
}

#[derive(Debug)]
enum ValidationError {
    MissingIdOnUpdate,
    MissingRequiredOnCreate(String),
    IdProvidedOnCreate,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ValidationError::MissingIdOnUpdate => {
                write!(fmt, "id required for updates")
            },
            ValidationError::MissingRequiredOnCreate(ref s) => {
                write!(fmt, "missing required field `{}`", s)
            },
            ValidationError::IdProvidedOnCreate => {
                write!(fmt, "id must not be provided for create")
            },
        }
    }
}

impl std::error::Error for ValidationError {
    fn description(&self) -> &str {
        match *self {
            ValidationError::MissingIdOnUpdate => "Missing id error",
            ValidationError::MissingRequiredOnCreate(_) => "Missing required field error",
            ValidationError::IdProvidedOnCreate => "Id provided on create error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            ValidationError::MissingIdOnUpdate => None,
            ValidationError::MissingRequiredOnCreate(_) => None,
            ValidationError::IdProvidedOnCreate => None,
        }
    }
}
