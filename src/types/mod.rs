mod user;
pub use user::User;
use serde::Deserialize;
use std::any::Any;

pub(crate) trait DataObject: std::fmt::Debug {
    fn id(&self) -> i64;
    fn data_type(&self) -> DataType;
    fn as_any(&self) -> &dyn Any;
}

pub trait DataVisitor {
    fn visit_user(&self, u: &User);
}

#[derive(Debug, Deserialize)]
pub(crate) enum DataType {
    #[serde(rename = "user")]
    User,
}

