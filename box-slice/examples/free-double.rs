use std::{fmt, slice};

#[derive(Clone)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释掉，看看会出什么问题
impl Drop for RawBuffer {
    #[inline]
    fn drop(&mut self) {
        let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
        drop(data)
    }
}

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];

    let buf: RawBuffer = data.into();
    let buf2: RawBuffer = buf.clone();

    // 因为 buf 允许 Copy，所以这里 Copy 了一份
    use_buffer(buf);

    let data: Vec<u8> = vec![1, 2, 3, 4];

    println!("buf to die: {:?}", buf2); // ! 用完之后再次被 free 了，说明实现了 clone，那么 drop 就会被运行多次，那么指定的内存也会被free 多次

    // buf 还能用
    // println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    // drop(buf)
}
