use serde::Deserialize;

mod user;
pub use user::User;
mod note;
pub use note::Note;

pub(crate) trait DataObject: std::fmt::Debug + Send + Sync {
    fn get_id(&self) -> i32;
    fn get_owner_author_id(&self) -> i32;
}

pub(crate) trait DataMeta {
    fn get_id_col() -> &'static str;
    fn get_owner_author_col() -> &'static str;
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
