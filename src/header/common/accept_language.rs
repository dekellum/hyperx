use language_tags::LanguageTag;
use header::QualityItem;

header! {
    /// `Accept-Language` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.5)
    ///
    /// The `Accept-Language` header field can be used by user agents to
    /// indicate the set of natural languages that are preferred in the
    /// response.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept-Language = 1#( language-range [ weight ] )
    /// language-range  = <language-range, see [RFC4647], Section 2.1>
    /// ```
    ///
    /// # Example values
    /// * `da, en-gb;q=0.8, en;q=0.7`
    /// * `en-us;q=1.0, en;q=0.5, fr`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{AcceptLanguage, LanguageTag, qitem, TypedHeaders};
    ///
    /// let mut headers = http::HeaderMap::new();
    /// let mut langtag: LanguageTag = LanguageTag::parse("en-US").unwrap();
    /// headers.encode(
    ///     &AcceptLanguage(vec![
    ///         qitem(langtag),
    ///     ])
    /// );
    /// ```
    ///
    /// ```
    /// # extern crate http;
    /// # extern crate hyperx;
    /// # #[macro_use] extern crate language_tags;
    /// # use hyperx::header::{AcceptLanguage, LanguageTag, QualityItem, q, qitem, TypedHeaders};
    /// #
    /// # fn main() {
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(
    ///     &AcceptLanguage(vec![
    ///         qitem(LanguageTag::parse("da").unwrap()),
    ///         QualityItem::new(LanguageTag::parse("en-US").unwrap(), q(800)),
    ///         QualityItem::new(LanguageTag::parse("en").unwrap(), q(700)),
    ///     ])
    /// );
    /// # }
    /// ```
    (AcceptLanguage, "Accept-Language") => (QualityItem<LanguageTag>)+

    test_accept_language {
        // From the RFC
        test_header!(test1, vec![b"da, en-gb;q=0.8, en;q=0.7"]);
        // Own test
        test_header!(
            test2, vec![b"en-US, en; q=0.5, fr"],
            Some(AcceptLanguage(vec![
                qitem("en-US".parse().unwrap()),
                QualityItem::new("en".parse().unwrap(), q(500)),
                qitem("fr".parse().unwrap()),
        ])));
    }
}

bench_header!(bench, AcceptLanguage,
              { vec![b"en-us;q=1.0, en;q=0.5, fr".to_vec()] });

standard_header!(AcceptLanguage, ACCEPT_LANGUAGE);
