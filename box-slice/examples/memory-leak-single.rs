fn main() {
    println!("{:p}", main as *const ());

    const LENGTH: usize = 20;

    let mut data: Vec<u8> = Vec::with_capacity(LENGTH);
    data.extend(&(0..data.capacity() as u8).collect::<Vec<_>>()[..]);
    for _ in 0..2 {
        data.pop();
    }

    let cap = data.capacity();
    let len = data.len();
    println!("data: {:p} cap: {} len: {}", &data, cap, len);

    let slice = data.into_boxed_slice();
    let length = slice.len();
    let ptr: *mut u8 = Box::into_raw(slice) as *mut u8;
    println!("content: {:?}", {
        let s = unsafe { std::slice::from_raw_parts_mut(ptr, cap) };
        s
    });
    println!("slice: ptr: {:?} length: {}", ptr, length);

    let _data = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, length)) };
    // let _data = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, length)) };
    drop(_data);

    let mut data: Vec<u8> = Vec::with_capacity(LENGTH);
    data.extend([1]);
    let cap = data.capacity();
    let len = data.len();
    println!("data: {:p} cap: {} len: {}", &data, cap, len);

    let slice = data.into_boxed_slice();
    let length = slice.len();
    let ptr: *mut u8 = Box::into_raw(slice) as *mut u8;
    println!("content: {:?}", unsafe {
        std::slice::from_raw_parts_mut(ptr, cap)
    });
    println!("slice: ptr: {:?} length: {}", ptr, length);

    let _data = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, length)) };
}
