use std::collections::HashMap;
use std::mem::size_of;

struct TestHash;

#[derive(Debug, PartialEq, Eq)]
struct TestKey {
    key: char,
}
impl TestKey {
    fn new(key: char) -> Self {
        Self { key }
    }
}
impl std::hash::Hash for TestKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        'a'.hash(state);
    }
}

fn main() {
    println!("main pointer: {:p}", main as *const ());
    println!(
        "size of HashMap<char, i32>: {}",
        size_of::<HashMap<TestKey, i32, TestHash>>()
    );

    let map: HashMap<TestKey, i32> = HashMap::new();
    let mut map = explain("empty", map);

    map.insert(TestKey::new('a'), 1);
    let mut map = explain("size1", map);

    map.insert(TestKey::new('b'), 2);
    let mut map = explain("size2", map);
    map.insert(TestKey::new('c'), 3);
    let mut map = explain("size3", map);

    map.insert(TestKey::new('d'), 4);
    let mut map = explain("size4", map);

    map.remove(&TestKey::new('a'));
    let mut map = explain("size3", map);

    map.remove(&TestKey::new('b'));
    map.shrink_to_fit();
    let mut map = explain("size2", map);

    map.insert(TestKey::new('a'), 1);
    map.insert(TestKey::new('b'), 2);
    let mut map = explain("size4", map);

    map.insert(TestKey::new('e'), 5);
    map.insert(TestKey::new('f'), 6);
    map.insert(TestKey::new('g'), 7);
    let mut map = explain("size7", map);

    map.insert(TestKey::new('f'), 8);
    explain("size8", map);
}

// HashMap 结构有两个 u64 的 RandomState，然后是四个 usize，
// 分别是 bucket_mask, ctrl, growth_left 和 items
// 我们 transmute 打印之后，再 transmute 回去
fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };

    #[allow(unused)]
    use hashbrown::HashMap;

    let ctrl = arr[0]; // *const u8 指针 最后一个 bucket 位置
    let bucket_mask = arr[1]; // 2^n - 1 容量
    let growth_left = arr[2]; // 剩下空位
    let items = arr[3]; // 已经使用的个数

    println!(
        "{}: ctrl 0x{:x}, bucket_mask 0x{:x}, growth_left: {:x}, items: {:x}",
        name, ctrl, bucket_mask, growth_left, items
    );

    // 打印出对应的地址
    let end = ctrl;
    let start = end - (size_of::<char>() + size_of::<i32>()) * (bucket_mask + 1);
    for i in 0..=(bucket_mask + 1) {
        let ptr = start + (size_of::<char>() + size_of::<i32>()) * i;
        print!("{:p}", ptr as *const u32);
        {
            let ptr = ptr as *const u32;
            unsafe {
                print!(" {:0>8}", format!("{:x}", *ptr));
            }
        }
        {
            let ptr = (ptr + size_of::<u32>()) as *const u32;
            unsafe {
                print!(" {:0>8}", format!("{:x}", *ptr));
            }
        }
        println!()
    }
    let ptr = end + (size_of::<char>() + size_of::<i32>());
    print!("{:p}", ptr as *const u32);
    {
        let ptr = ptr as *const u32;
        unsafe {
            print!(" {:0>8}", format!("{:x}", *ptr));
        }
    }
    {
        let ptr = (ptr + size_of::<u32>()) as *const u32;
        unsafe {
            print!(" {:0>8}", format!("{:x}", *ptr));
        }
    }
    println!();

    unsafe { std::mem::transmute(arr) }
}
