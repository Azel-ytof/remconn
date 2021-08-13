use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct SshError {
    error_message: String,
}

impl SshError {
    pub fn new(error_message: String) -> Self {
        Self { error_message }
    }
}

impl Display for SshError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "A problem occurred on SSH crate : {}",
            self.error_message
        )
    }
}

impl std::error::Error for SshError {}
