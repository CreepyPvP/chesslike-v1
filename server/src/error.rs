use std::io;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
