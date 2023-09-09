use std::{error::Error as StdError, fmt};

pub type BoxedError = Box<dyn StdError>;

#[derive(Clone, Debug)]
pub struct StringError(pub String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl StdError for StringError {}
