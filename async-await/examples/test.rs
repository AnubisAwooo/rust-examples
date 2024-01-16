use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(5));
    let guard = &mutex.lock().unwrap();
    {
        let thread1 = thread::spawn(move || {
            println!("{:?}", *guard);
        });
        thread1.join().unwrap();
    }
}

fn main2() {
    let mutex = Arc::new(Mutex::new(5));
    let guard = mutex.lock().unwrap();
    // scope 共享语义
    thread::scope(|f| {
        f.spawn(|| {
            println!("{:?}", *guard);
        });
    })
}

// Send 代表对象的所有权可以被转移到其他线程（包括但不限于可以在别的线程执行drop）// ! 其他线程甚至可以 drop
// Sync 代表对象的不可变引用可以被转移到其他线程（不会执行任何操作改变当前数据）// ! 其他线程仅仅可以访问, 会不会出现主线程提前释放问题?
// T: Sync 等价于 &T: Send

fn main3() {
    #[derive(Debug)]
    struct T;

    unsafe impl Sync for T {}

    let t = T;
    let t2 = &t;

    thread::spawn(move || {
        println!("{:?}", t2);
    });
}
