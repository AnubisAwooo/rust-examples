use anyhow::Result;
use futures::Future;
use std::sync::Mutex;
use std::{sync::Arc, time::Duration};

struct DB;

impl DB {
    // 假装在 commit 数据
    // async fn commit(&mut self) -> Result<usize> {
    //     Ok(42)
    // }

    fn commit(&mut self) -> impl Future<Output = Result<usize>> {
        async move { Ok(42) }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db1 = Arc::new(Mutex::new(DB));
    let _db2 = Arc::clone(&db1);
    let db3 = Arc::clone(&db1);

    // tokio::spawn(async move {
    //     let mut db = db1.lock().unwrap();
    //     // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
    //     // 只能用 tokio::sync::Mutex
    //     let affected = db.commit().await?;
    //     println!("db1: Total affected rows: {}", affected);
    //     Ok::<_, anyhow::Error>(())
    // });

    // ! async 执行块里面按照 await 分块, 每个块可能是不同线程来执行, 那么没有 Send 的对象,是不能够跨块(线程)接着使用的
    tokio::spawn(async move {
        let mut _db = db3.lock().unwrap();
        // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
        // 只能用 tokio::sync::Mutex
        // drop(_db);
        let _affected = async move {}.await;
        // println!("db1: Total affected rows: {:?}", affected);
        // Ok::<_, anyhow::Error>(())
    });

    // tokio::spawn(async move {
    //     let mut db = db2.lock().unwrap();
    //     let affected = db.commit().await?;
    //     println!("db2: Total affected rows: {}", affected);

    //     Ok::<_, anyhow::Error>(())
    // });

    // 让两个 task 有机会执行完
    tokio::time::sleep(Duration::from_millis(1)).await;

    Ok(())
}
