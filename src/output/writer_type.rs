use std::error::Error;

pub type WriterResult<T> = Result<T, Box<dyn Error>>;