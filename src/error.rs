use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustyError {
    #[error("Invalid topic")]
    InvalidTopic,

    #[error("Internal error: {0}")]
    Internal(String),
}
