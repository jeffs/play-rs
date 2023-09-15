#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Timestamp(String);

impl<T: Into<String>> From<T> for Timestamp {
    fn from(value: T) -> Self {
        Timestamp(value.into())
    }
}
