mod abi;

pub use abi::*;
use bytes::{Bytes, BytesMut};
use prost::Message;

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}
impl TryFrom<BytesMut> for Response {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}

impl From<Response> for Bytes {
    fn from(value: Response) -> Self {
        let mut bytes = BytesMut::new();
        value.encode(&mut bytes).unwrap();
        bytes.freeze()
    }
}

impl From<Request> for Bytes {
    fn from(value: Request) -> Self {
        let mut bytes = BytesMut::new();
        value.encode(&mut bytes).unwrap();
        bytes.freeze()
    }
}

impl Request {
    pub fn new_get(key: &str) -> Self {
        Self {
            command: Some(request::Command::Get(RequestGet { key: key.into() })),
        }
    }
    pub fn new_put(key: &str, value: &[u8]) -> Self {
        Self {
            command: Some(request::Command::Put(RequestPut {
                key: key.into(),
                value: value.into(),
            })),
        }
    }
    pub fn new_delete(key: &str) -> Self {
        Self {
            command: Some(request::Command::Delete(RequestDelete { key: key.into() })),
        }
    }
}

impl Response {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            code: 0,
            key,
            value,
        }
    }
    pub fn not_found(key: String) -> Self {
        Self {
            code: 404,
            key,
            ..Default::default()
        }
    }
}
