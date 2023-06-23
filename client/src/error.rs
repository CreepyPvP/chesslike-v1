use std::io;

#[derive(Debug)]
pub enum ClientError {
    IoError(String),    
}

impl From<io::Error> for ClientError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}
