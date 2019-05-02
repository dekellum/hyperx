//! Implementation module for various _compat_ features with the _http_ crate.

use std::fmt::Display;

use http::header::HeaderMap;

use ::Result;
use super::Header;

/// A trait for the "standard" headers that have an associated `HeaderName`
/// constant in the `http` crate.
pub trait StandardHeader: Header + Sized {
    /// The `HeaderName` from the _http_ crate for this header.
    fn http_header_name() -> ::http::header::HeaderName;
}

/// Extension trait for `decode` (parsing) and `encode` (serialization) of
/// typed headers from/to a collection of headers such as `HeaderMap`.
pub trait TypedHeaders {
    /// Decode and return `Header` type H or `Error::Header`.
    ///
    /// `Error::Header` is returned on failed parse, or for a single-valued
    /// Header type, if no values or multiple values are found in the
    /// collection.  Multi-valued header types such as `ContentEncoding` will
    /// instead return an empty list value if no values are found.  To
    /// distinguish the not found case, use `try_decode` instead.
    fn decode<H>(&self) -> Result<H>
        where H: StandardHeader;

    /// Decode and return `Header` type H or `Error::Header` if found, or
    /// return `None` if not found.
    ///
    /// This variant will return `Option::None` if no header with the
    /// associated key (`HeaderName`) is found in the collection. If the
    /// collection does contain such a key, it will return the header type H or
    /// `Error::Header`.
    fn try_decode<H>(&self) -> Option<Result<H>>
        where H: StandardHeader;

    /// Encode and write the specified typed header value in the collection.
    ///
    /// Uses the `Display` format of the provided header value to write a single
    /// header. This will overwrite any preexisting values with the same
    /// key (`HeaderName`). Use `encode_append` instead to avoid this.
    fn encode<H>(&mut self, value: &H)
        where H: StandardHeader + Display;

    /// Encode and append the specified typed header value into the collection.
    ///
    /// Uses the `Display` format of the provided header value to append a
    /// single header. If the collection previously had a value for the same
    /// key, the additional value is appended to the end.
    fn encode_append<H>(&mut self, value: &H)
        where H: StandardHeader + Display;
}

impl TypedHeaders for HeaderMap {
    fn decode<H>(&self) -> Result<H>
        where H: StandardHeader
    {
        let vals = self.get_all(H::http_header_name());
        H::parse_header(&vals)
    }

    fn try_decode<H>(&self) -> Option<Result<H>>
        where H: StandardHeader
    {
        let hname = H::http_header_name();
        if self.contains_key(&hname) {
            let vals = self.get_all(&hname);
            Some(H::parse_header(&vals))
        } else {
            None
        }
    }

    fn encode<H>(&mut self, val: &H)
        where H: StandardHeader + Display
    {
        self.insert(
            H::http_header_name(),
            val.to_string().parse().expect("header value"));
    }

    fn encode_append<H>(&mut self, val: &H)
        where H: StandardHeader + Display
    {
        self.append(
            H::http_header_name(),
            val.to_string().parse().expect("header value"));
    }
}

#[cfg(test)]
mod tests {
    use http;
    use super::TypedHeaders;
    use ::header::{ContentEncoding, ContentLength, Encoding, Te, ETag};

    #[cfg(feature = "nightly")]
    use ::header::Header;

    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_empty_decode() {
        let hmap = http::HeaderMap::new();
        let len = hmap.decode::<ContentLength>();
        assert!(len.is_err());
    }

    #[test]
    fn test_empty_decode_etag() {
        let hmap = http::HeaderMap::new();
        let len = hmap.decode::<ETag>();
        assert!(len.is_err());
    }

    #[test]
    fn test_empty_decode_te() {
        let hmap = http::HeaderMap::new();
        let te = hmap.decode::<Te>().unwrap();
        assert_eq!(te, Te(vec![]));
    }

    #[test]
    fn test_empty_decode_content_encoding() {
        let hmap = http::HeaderMap::new();
        let ce = hmap.decode::<ContentEncoding>().unwrap();
        assert_eq!(ce, ContentEncoding(vec![]));
    }

    #[test]
    fn test_empty_try_decode() {
        let hmap = http::HeaderMap::new();
        let len = hmap.try_decode::<ContentLength>();
        assert!(len.is_none());
    }

    #[test]
    fn test_empty_try_decode_te() {
        let hmap = http::HeaderMap::new();
        let te = hmap.try_decode::<Te>();
        assert!(te.is_none());
    }

    #[test]
    fn test_decode() {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        let len: ContentLength = hmap.decode().unwrap();
        assert_eq!(*len, 11);
    }

    #[test]
    fn test_encode_decode() {
        let mut hmap = http::HeaderMap::new();
        hmap.encode(&ContentLength(11));
        let len: ContentLength = hmap.decode().unwrap();
        assert_eq!(*len, 11);
    }

    #[test]
    fn test_encode_append() {
        let mut hmap = http::HeaderMap::new();
        hmap.encode_append(
            &ContentEncoding(vec![Encoding::Identity]));
        hmap.encode_append(
            &ContentEncoding(vec![Encoding::Gzip, Encoding::Chunked]));
        let ce: ContentEncoding = hmap.decode().unwrap();
        assert_eq!(
            *ce,
            vec![Encoding::Identity, Encoding::Gzip, Encoding::Chunked]);
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_0_get_parse_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let vals = hmap.get_all(http::header::CONTENT_LENGTH);
            let len = ContentLength::parse_header(&vals).unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_0_get_parse_int_one(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let val = hmap.get(http::header::CONTENT_LENGTH).unwrap();
            let len = ContentLength::parse_header(&val).unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_1_decode_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let len: ContentLength = hmap.decode().unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_1_try_decode_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let len: ContentLength = hmap.try_decode().unwrap().unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_2_get_orig_int(b: &mut Bencher) {
        let mut hdrs = ::header::Headers::new();
        hdrs.set_raw("content-length", "11");
        b.iter(|| {
            let len: &ContentLength = hdrs.get().unwrap();
            assert_eq!(**len, 11);
        })
    }
}
