mod user;
use serde::Deserialize;
pub use user::User;

pub(crate) trait DataObject: std::fmt::Debug + Send + Sync {}

pub trait DataVisitor {
    fn visit_user(&self, u: &User);
}

#[derive(Debug, Deserialize)]
pub(crate) enum DataType {
    #[serde(rename = "user")]
    User,
}
