mod user;
pub use user::User;
use serde::Deserialize;

pub trait DataObject: Clone {
    fn id(&self) -> i64;
}

#[derive(Debug, Deserialize)]
pub(crate) enum DataType {
    #[serde(rename = "user")]
    User,
}
