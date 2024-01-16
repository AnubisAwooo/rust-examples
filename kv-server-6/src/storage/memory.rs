use crate::{KvError, KvPair, Storage, StorageIter, Value};
use dashmap::{mapref::one::Ref, DashMap};

/// 使用 DashMap 构建的 MemTable，实现了 Storage trait
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    /// 创建一个缺省的 MemTable
    pub fn new() -> Self {
        Self::default()
    }

    /// 如果名为 name 的 hash table 不存在，则创建，否则返回
    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<KvPair>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table
            .iter()
            .map(|v| KvPair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError> {
        // 0 自己写的方法, 是不符合迭代器的, 不过 clone 貌似也不符合迭代器啊, 不收集不运行
        // let table = self.get_or_create_table(table);
        // let iter: Vec<KvPair> = table
        //     .iter()
        //     .map(|v| KvPair::new(v.key(), v.value().clone()))
        //     .collect();
        // let iter = Box::new(iter.into_iter());
        // Ok(iter)

        // 1 table clone 了, 但是是局部变量, iter 持有这个局部变量, 导致 iter 的对象无法移除当前栈帧
        // 使用 clone() 来获取 table 的 snapshot
        // let table = self.get_or_create_table(table).clone();
        // let iter = table
        //     .iter() // ! 这就是问题所在, iter 是取引用的
        //     .map(|v| KvPair::new(v.key(), v.value().clone()));
        // Ok(Box::new(iter)) // <-- 编译出错

        // 2 把整个 clone 的 table
        // 使用 clone() 来获取 table 的 snapshot
        // let table = self.get_or_create_table(table).clone();
        // let iter = table.into_iter().map(|data| data.into());
        // Ok(Box::new(iter))

        // 3 返回包装的对象
        // 使用 clone() 来获取 table 的 snapshot
        let table = self.get_or_create_table(table).clone();
        let iter = StorageIter::new(table.into_iter()); // 这行改掉了
        Ok(Box::new(iter))
    }
}

impl From<(String, Value)> for KvPair {
    fn from(data: (String, Value)) -> Self {
        KvPair::new(data.0, data.1)
    }
}
