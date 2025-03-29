mod user;
pub use user::User;

pub trait DataObject: Clone {
    fn id(&self) -> i64;
}
