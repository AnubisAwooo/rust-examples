use crate::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),
    #[error("Frame is larger than max size")]
    FrameError,
    #[error("Command is invalid: `{0}`")]
    InvalidCommand(String),
    #[error("Cannot convert value {0:?} to {1}")]
    ConvertError(Value, &'static str),
    #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {3}")]
    StorageError(&'static str, String, String, String),

    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),
    #[error("Failed to access sled db")]
    SledError(#[from] sled::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl PartialEq for KvError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NotFound(l0, l1), Self::NotFound(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::InvalidCommand(l0), Self::InvalidCommand(r0)) => l0 == r0,
            (Self::ConvertError(l0, l1), Self::ConvertError(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::StorageError(l0, l1, l2, l3), Self::StorageError(r0, r1, r2, r3)) => {
                l0 == r0 && l1 == r1 && l2 == r2 && l3 == r3
            }
            (Self::EncodeError(l0), Self::EncodeError(r0)) => l0 == r0,
            (Self::DecodeError(l0), Self::DecodeError(r0)) => l0 == r0,
            (Self::SledError(l0), Self::SledError(r0)) => l0 == r0,
            (Self::IoError(_l0), Self::IoError(_r0)) => false,
            (Self::Internal(l0), Self::Internal(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
