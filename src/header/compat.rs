use std::string::ToString;
use http::header::HeaderMap;
use ::Result;
use super::Header;

/// A trait for standard headers with constant names.
pub trait StandardHeader: Header + Sized + ToString {
    /// The http crate HeaderName
    fn http_header_name() -> http::header::HeaderName;
}

pub trait TypedHeaders {
    fn decode<H>(&self) -> Result<H>
        where H: StandardHeader;

    fn decode_any<H>(&self) -> Result<H>
        where H: Header;

    fn encode<H>(&mut self, val: &H)
        where H: StandardHeader;
}

impl TypedHeaders for HeaderMap {
    fn decode<H>(&self) -> Result<H>
        where H: StandardHeader
    {
        let vals = self.get_all(H::http_header_name());
        H::parse_header(&vals)
    }

    fn decode_any<H>(&self) -> Result<H>
        where H: Header
    {
        let vals = self.get_all(H::header_name());
        // FIXME: Perf: AsHeaderName for &str validates and down cases
        H::parse_header(&vals)
    }

    fn encode<H>(&mut self, val: &H)
        where H: StandardHeader
    {
        self.insert(
            H::http_header_name(),
            val.to_string().parse().expect("header value"));
    }
}

#[cfg(test)]
mod tests {
    use http;
    use super::TypedHeaders;
    use ::header::ContentLength;

    #[cfg(feature = "nightly")]
    use ::header::Header;

    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_empty_decode() {
        let hmap = http::HeaderMap::new();
        let len = hmap.decode::<ContentLength>();
        assert!(len.is_err());
        println!("{:?}", len);
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
    fn bench_2_get_orig_int(b: &mut Bencher) {
        let mut hdrs = ::header::Headers::new();
        hdrs.set_raw("content-length", "11");
        b.iter(|| {
            let len: &ContentLength = hdrs.get().unwrap();
            assert_eq!(**len, 11);
        })
    }
}
