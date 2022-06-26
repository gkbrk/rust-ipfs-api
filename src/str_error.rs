use std::error::Error;
use std::fmt::{Debug, Display};

pub struct StrError<'a> {
    msg: &'a str,
}

impl<'a> StrError<'a> {
    pub fn from_str(error: &str) -> Box<StrError> {
        Box::new(StrError { msg: error.into() })
    }
}

impl<'a> Display for StrError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StrError(\"{}\")", self.msg)
    }
}

impl<'a> Debug for StrError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrError")
            .field("Message", &self.msg)
            .finish()
    }
}

impl<'a> Error for StrError<'a> {}
