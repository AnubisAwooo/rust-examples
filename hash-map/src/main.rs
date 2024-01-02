use std::collections::HashMap;

fn main() {
    println!("{}", std::mem::size_of::<HashMap<char, i32>>());

    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert('a', 1);
    let mut map = explain("add 1", map);
    map.insert('b', 2);
    map.insert('c', 3);

    let mut map = explain("add 3", map);

    map.insert('d', 4);

    let mut map = explain("add 4", map);

    map.remove(&'a');

    let mut map = explain("final", map);

    map.remove(&'b');

    map.shrink_to_fit();

    explain("final", map);
}

// HashMap 结构有两个 u64 的 RandomState，然后是四个 usize，
// 分别是 bucket_mask, ctrl, growth_left 和 items
// 我们 transmute 打印之后，再 transmute 回去
fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    // println!(
    //     "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {:x}, items: {:x} {:x} {:x}",
    //     name, arr[2], arr[3], arr[4], arr[5], arr[0], arr[1]
    // );
    // 3 -> items
    // 1 -> bucket_mask
    // 2 -> growth_left
    // 0 -> ctrl

    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {:x}, items: {:x} {:x} {:x}",
        name, arr[1], arr[0], arr[2], arr[3], arr[4], arr[5]
    );

    unsafe { std::mem::transmute(arr) }
}
