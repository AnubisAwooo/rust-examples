fn main() {
    let mut s = "我爱你！中国".to_string();
    let r = s.as_mut();

    if let Some((s1, s2)) = split_mut(r, '！') {
        println!("s1: {}, s2: {}", s1, s2);
    }
}

fn split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);

    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();

        // SAFETY: pos 是 find 得到的，它位于字符的边界处，同样 pos + sep_len 也是如此
        // 所以以下代码是安全的
        unsafe { (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..len)) }
    })
}

fn split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    split(s, sep).map(|(s1, s2)| {
        let (ptr1, len1) = unsafe { std::mem::transmute(s1) };
        let (ptr2, len2) = unsafe { std::mem::transmute(s2) };
        let s1 = unsafe {
            std::str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(ptr1, len1))
        };
        let s2 = unsafe {
            std::str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(ptr2, len2))
        };
        (s1, s2)
    })
}
