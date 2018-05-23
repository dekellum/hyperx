use std::ops::Deref;
use std::str;

use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByteStr(Bytes);

impl ByteStr {
    pub fn from_static(s: &'static str) -> ByteStr {
        ByteStr(Bytes::from_static(s.as_bytes()))
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }

    pub fn insert(&mut self, idx: usize, ch: char) {
        let mut s = self.as_str().to_owned();
        s.insert(idx, ch);
        let bytes = Bytes::from(s);
        self.0 = bytes;
    }

    #[cfg(feature = "compat")]
    pub fn into_bytes(self) -> Bytes {
        self.0
    }
}

impl Deref for ByteStr {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for ByteStr {
    fn from(s: &'a str) -> ByteStr {
        ByteStr(Bytes::from(s))
    }
}
