use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum DirError {
    Message(String)
}

impl Error for DirError {}

impl Display for DirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            Self::Message(msg) => msg
        });
    }
}