// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{error::Error, fmt::Display};

pub type AoCResult<T> = Result<T, AoCError>;

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

    pub fn new_from_parseerror<E: Into<Box<dyn Error>>>(cause: E) -> AoCError {
        Self::new_with_cause("failed to parse:", cause)
    }

    pub fn new_from_char(value: char) -> Self {
        Self::new(format!("Unexpected character '{value}'"))
    }
}
