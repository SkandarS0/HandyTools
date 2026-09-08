use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordCliError {
    #[error(transparent)]
    Generator(#[from] hnd_password_generator::PasswordGeneratorError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("password count must be greater than 0")]
    ZeroCount,
}
