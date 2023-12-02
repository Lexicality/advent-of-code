use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct AoCError {
    cause: Option<Box<dyn Error>>,
    message: String,
}

impl Display for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AoCError {
    fn cause(&self) -> Option<&dyn Error> {
        self.cause.as_deref()
    }
}

impl AoCError {
    pub fn new<S: Into<String>>(message: S) -> AoCError {
        AoCError {
            cause: None,
            message: message.into(),
        }
    }

    pub fn new_with_cause<S: Into<String>, E: Into<Box<dyn Error>>>(
        message: S,
        cause: E,
    ) -> AoCError {
        AoCError {
            cause: Some(cause.into()),
            message: message.into(),
        }
    }
}
