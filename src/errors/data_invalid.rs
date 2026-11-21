use std::io::{Error, ErrorKind};
use std::error::Error as err;

pub fn invalid_err(message: impl Into<String>) -> Box<dyn err> {
    Box::new(Error::new(
        ErrorKind::InvalidData,
        message.into(),
    ))
}