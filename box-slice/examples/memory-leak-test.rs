fn release(data: Vec<u8>) {
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

fn main() {
    println!("{:p}", main as *const ());

    let mut data = Vec::with_capacity(17);
    data.extend(&(0..data.capacity() as u8).collect::<Vec<_>>()[..]);
    data.pop();
    data.pop();
    data.pop();

    release(data);

    let mut data = Vec::with_capacity(17);
    data.extend([1, 2, 3, 4, 5]);
    let mut data2 = Vec::with_capacity(17);
    data2.extend([1, 2, 3, 4]);

    release(data);
    release(data2);
}
