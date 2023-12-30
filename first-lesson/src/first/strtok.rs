pub fn strtok<'a>(s: &mut &'a str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(i) => {
            let prefix = &s[..i];
            let suffix = &s[(i + pat.len_utf8())..];
            *s = suffix;
            prefix
        }
        None => {
            let r = *s;
            *s = "";
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strtok_test() {
        let mut s = "hello world";
        assert_eq!(s.find(' '), Some(5));

        let t = strtok(&mut s, ' ');
        drop(s);
        assert_eq!(t, "hello");
        assert_eq!(s, "world");
    }
}
