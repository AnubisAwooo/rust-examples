use std::path::Path;

use rocksdb::DB;

use crate::{KvError, KvPair, Storage, StorageIter, Value};

#[derive(Debug)]
pub struct RocksDb(DB);

impl RocksDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let db = DB::open_default(path).unwrap();
        Self(db)
    }

    // 在 sled_db 里，因为它可以 scan_prefix，我们用 prefix
    // 来模拟一个 table。当然，还可以用其它方案。
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }

    // 遍历 table 的 key 时，我们直接把 prefix: 当成 table
    fn get_table_prefix(table: &str) -> String {
        format!("{}:", table)
    }
}

/// 把 Option<Result<T, E>> flip 成 Result<Option<T>, E>
/// 从这个函数里，你可以看到函数式编程的优雅
fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

impl TryFrom<Vec<u8>> for Value {
    type Error = KvError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(prost::Message::decode(&value[..])?)
    }
}

impl From<rocksdb::Error> for KvError {
    fn from(value: rocksdb::Error) -> Self {
        KvError::Internal(value.to_string())
    }
}

impl Storage for RocksDb {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = RocksDb::get_full_key(table, key);
        let result = self.0.get(name.as_bytes())?.map(|v| v.try_into());
        flip(result)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let name = RocksDb::get_full_key(table, &key);
        let data: Vec<u8> = value.try_into()?;

        let old = self.get(table, &key)?;

        self.0.put(name, data)?;

        Ok(old)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let name = RocksDb::get_full_key(table, &key);

        Ok(self.0.key_may_exist(name))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = RocksDb::get_full_key(table, &key);

        let old = self.get(table, &key)?;

        self.0.delete(name)?;

        Ok(old)
    }

    fn get_all(&self, table: &str) -> Result<Vec<KvPair>, KvError> {
        let prefix = RocksDb::get_table_prefix(table);
        let result = self.0.prefix_iterator(prefix).map(|v| v.into()).collect();

        Ok(result)
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError> {
        let prefix = RocksDb::get_table_prefix(table);

        let iter = StorageIter::new(
            self.0
                .prefix_iterator(prefix)
                .map(|v| v.into())
                .collect::<Vec<KvPair>>()
                .into_iter(),
        );

        Ok(Box::new(iter))
    }
}

impl From<Result<(Box<[u8]>, Box<[u8]>), rocksdb::Error>> for KvPair {
    fn from(v: Result<(Box<[u8]>, Box<[u8]>), rocksdb::Error>) -> Self {
        match v {
            Ok((k, v)) => match v.as_ref().try_into() {
                Ok(v) => KvPair::new(ivec_to_key(k.as_ref()), v),
                Err(_) => KvPair::default(),
            },
            _ => KvPair::default(),
        }
    }
}

fn ivec_to_key(ivec: &[u8]) -> &str {
    let s = std::str::from_utf8(ivec).unwrap();
    let mut iter = s.split(":");
    iter.next();
    iter.next().unwrap()
}
