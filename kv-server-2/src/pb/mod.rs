use bytes::BytesMut;
use prost::Message;

mod abi;

pub use abi::command_request::*;
pub use abi::*;

impl TryFrom<BytesMut> for CommandRequest {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}

// impl From<CommandRequest> for BytesMut {
//     fn from(value: CommandRequest) -> Self {
//         let mut bytes = BytesMut::new();
//         value.encode(&mut bytes).unwrap();
//         bytes
//     }
// }

// impl TryFrom<CommandResponse> for BytesMut {
//     type Error = prost::EncodeError;

//     fn try_from(value: CommandResponse) -> Result<Self, Self::Error> {
//         let mut bytes = BytesMut::new();
//         value.encode(&mut bytes)?;
//         Ok(bytes)
//     }
// }

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        CommandResponse {
            status: 0,
            message: "success".into(),
            values: vec![value],
            pairs: vec![],
        }
    }
}

/// 对 Command 的处理的抽象
pub trait CommandService {
    /// 处理 Command，返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

// 从 Request 中得到 Response，目前处理 HGET/HGETALL/HSET
// pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
//     match cmd.request_data {
//         Some(RequestData::Hget(param)) => param.execute(store),
//         Some(RequestData::Hgetall(param)) => param.execute(store),
//         Some(RequestData::Hset(param)) => param.execute(store),
//         None => KvError::InvalidCommand("Request has no data".into()).into(),
//         _ => KvError::Internal("Not implemented".into()).into(),
//     }
// }

pub type KvError = ();

/// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从 HashTable 中删除一个 key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Vec<KvPair>, KvError>;
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError>;
}
