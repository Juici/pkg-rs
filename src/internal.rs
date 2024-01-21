#[track_caller]
pub const fn slice_str(s: &'static str, start: usize, end: usize) -> &'static str {
    assert!(start <= end, "start cannot be greater than end");
    assert!(end <= s.len(), "end cannot be greater than the length of the string");

    let len = end - start;

    // SAFETY: `start` is within the bounds of `s` at a character boundary.
    let ptr = unsafe { s.as_ptr().add(start) };

    // SAFETY: `ptr` is valid for read of `len` bytes.
    let slice = unsafe { ::core::slice::from_raw_parts(ptr, len) };

    match ::core::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => panic!("slice results in invalid utf-8"),
    }
}

pub const fn split_on(s: &'static str, b: u8) -> (&'static str, &'static str) {
    let bytes = s.as_bytes();

    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b {
            break;
        }
        i += 1;
    }

    if i == bytes.len() {
        (s, "")
    } else {
        (slice_str(s, 0, i), slice_str(s, i + 1, s.len()))
    }
}
