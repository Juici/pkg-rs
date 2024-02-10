use crate::str::split_on;

// Cargo separates the names of authors with the colon ':' character.
//
// eg. "author1:author2:author3"
const SEP: u8 = b':';

struct IterAuthors<'a>(&'a str);

impl<'a> IterAuthors<'a> {
    const fn next(mut self) -> Option<(&'a str, IterAuthors<'a>)> {
        while !self.0.is_empty() {
            let (author, remaining) = split_on(self.0, SEP);

            self.0 = remaining;

            if !author.is_empty() {
                return Some((author, self));
            }
        }

        None
    }

    const fn count(mut self) -> usize {
        let mut n = 0;

        while let Some((_, iter)) = self.next() {
            n += 1;
            self = iter;
        }

        n
    }
}

#[doc(hidden)]
#[must_use]
pub const fn count_authors(authors_str: &str) -> usize {
    IterAuthors(authors_str).count()
}

#[doc(hidden)]
#[must_use]
#[track_caller]
pub const fn authors<const N: usize>(authors_str: &str) -> [&str; N] {
    assert!(count_authors(authors_str) == N, "count_authors(authors_str) != N");

    let mut authors = [""; N];

    let mut i = 0;
    let mut iter = IterAuthors(authors_str);

    while let Some((author, new_iter)) = iter.next() {
        authors[i] = author;

        i += 1;
        iter = new_iter;
    }

    authors
}

#[doc(hidden)]
#[must_use]
pub const fn join_authors_buf_len(authors: &[&str], sep: &str) -> usize {
    let mut len = 0;
    let mut i = 0;

    while i < authors.len() {
        if i > 0 {
            i += sep.len();
        }
        len += authors[i].len();

        i += 1;
    }

    len
}

#[doc(hidden)]
#[must_use]
#[track_caller]
pub const fn join_authors<const BUF_LEN: usize>(authors: &[&str], sep: &str) -> [u8; BUF_LEN] {
    assert!(
        join_authors_buf_len(authors, sep) == BUF_LEN,
        "join_authors_buf_len(authors, sep) != BUF_LEN",
    );

    let mut buf = [0; BUF_LEN];

    let mut offset = 0;
    let mut i = 0;

    while i < authors.len() {
        let author = authors[i];

        // Add a separator if this is not the first author.
        if i > 0 {
            let mut j = 0;

            // Copy the bytes of `sep` into the buffer.
            while j < sep.len() {
                buf[offset] = sep.as_bytes()[j];

                offset += 1;
                j += 1;
            }
        }

        {
            let mut j = 0;

            // Copy the bytes of `author` into the buffer.
            while j < author.len() {
                buf[offset] = author.as_bytes()[j];

                offset += 1;
                j += 1;
            }
        }

        i += 1;
    }

    buf
}
