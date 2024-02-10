use core::str;

#[track_caller]
pub const fn slice(s: &str, start: usize, end: usize) -> &str {
    assert!(start <= end, "start cannot be greater than end");
    assert!(end <= s.len(), "end cannot be greater than the length of the string");

    let bytes = s.as_bytes();

    let (bytes, _) = bytes.split_at(end);
    let (_, bytes) = bytes.split_at(start);

    match str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => panic!("slice results in invalid utf-8"),
    }
}

pub const fn split_on(s: &str, byte: u8) -> (&str, &str) {
    match find_byte(s.as_bytes(), byte) {
        Some(index) => (slice(s, 0, index), slice(s, index + 1, s.len())),
        None => (s, ""),
    }
}

pub const fn find_byte(bytes: &[u8], byte: u8) -> Option<usize> {
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == byte {
            return Some(i);
        }
        i += 1;
    }

    None
}
