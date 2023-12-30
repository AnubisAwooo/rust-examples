use std::{fmt, slice};

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

        println!("data: cap: {:p}  len: {}", &data, data.len());
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
    println!("{:p}", main as *const ());

    let mut data = Vec::with_capacity(1025);
    data.extend([1, 2, 3, 4]);
    println!("data p: {:p} ", &data);
    let buf: RawBuffer = data.into();
    println!("data p: {:p} {:?}", &buf, buf.ptr);
    use_buffer(buf);

    let mut data = Vec::with_capacity(1024);
    let mut data2 = Vec::with_capacity(1024);
    data.extend([1, 2, 3, 4,5]);
    data2.extend([1, 2, 3, 4]);
    println!("data p: {:p} ", &data);
    println!("data2 p: {:p} ", &data2);
    let buf: RawBuffer = data.into();
    println!("data p: {:p} {:?}", &buf, buf.ptr);
    use_buffer(buf);
    let buf2: RawBuffer = data2.into();
    println!("data2 p: {:p} {:?}", &buf2, buf2.ptr);
    use_buffer(buf2);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    // drop(buf)
}
