## 0.16.0 (TBD)

* Upgrade (unconstrain) _cfg-if_ dependency to (0.1.10 is MSRV 1.31.0)

* Upgrade _unicase_ to (min) 2.1.0 to avoid compile failures with older
  releases.

* Upgrade to _percent-encoding_ 2.1.0 \w API changes, MSRV 1.33.0 (#15)

* MSRV is now 1.33.0, based on above upgrades.

## 0.15.2 (2019-10-1)

* Constrain transitive _cfg-if_ dependency to <0.1.10 to preserve MSRV 1.27.2.

* Narrow various other dependencies for future reliability.  We may
  subsequently make PATCH releases which _broaden_ private or public
  dependencies to include new releases found compatible.

## 0.15.1 (2019-6-3)

* Fix build.rs for `rustc --version` not including git metadata (alyssais #14)

## 0.15.0 (2019-5-8)

* Add a `TypedHeaders` extension trait providing more convenient generic
  encode/decode methods to `http::HeaderMap` for _hyperx_ typed headers,
  implemented using a new `StandardHeader` trait and `standard_header!` macro,
  with an associate function for the `HeaderName` constants of the _http_
  crate. (#13)

* Add reference based `impl From<&'a Headers> for http::HeaderMap` for symmetry
  and performance, e.g. avoiding a `clone`. (#13)

* Increase MSRV to 1.27.2, which enables us to revert a CI workaround for the
  fact that base64 0.10.1 was released with this same MSRV. (#10 #12)

* Add a build.rs to check MSRV and fail fast with a clear error when older
  rustc versions are used. (#12)

## 0.14.0 (2019-1-4)

* Update the signature of `Header::parse_header` to be generic over types
  implementing a new `RawLike` trait, which includes the existing local `Raw`
  type as well as _http_ crate types `HeaderValue` and (`HeaderMap::get_all`)
  `GetAll`. This avoids an allocation when directly parsing from these later
  types.

  _Expected Breakage_: Any 3rd-party custom headers directly implementing
  `parse_header` will need to change accordingly on upgrade. Also `Into`
  conversions to `Raw` now frequently need to be type annotated. (#8)

* Improve header module rustdoc, including with parsing usage for the above.

## 0.13.2 (2019-1-2)

* Remove un-exported, and unused as of 0.13.1, `uri` module and related code.

* Broaden base64 dependency to include 0.10.0, passing tests.

* Silence a deprecation warning for `str::trim_right_matches` until the minimum
  rust version is updated to 1.30.0.

## 0.13.1 (2018-6-26)

* Remove `error::UriError` re-export and `error::Canceled` which are unused
  internally and where not exported from this crate. (#5)

## 0.13.0 (2018-6-18)

* Remove variants from `hyperx::Error` which are unused by the header
  module. Exhaustive matching has been discouraged for this enum, but this is
  still a potential breaking change. (dekellum #2)

* Add an alternative, by reference `From<&http::HeaderMap>` for `Headers`.
  (DarrenTsung #3)

## 0.12.0 (2018-6-1)

Forked from hyper 0.11.27, e*x*tracting the typed *header* module
from [hyperium/hyper@76fdbcf2], 0.11.x branch, preserved here as
[76fdbcf2].

## Prior Releases

See [hyper's CHANGELOG] for prior updates pertaining to the headers
sub-module.

[hyper's CHANGELOG]: https://github.com/hyperium/hyper/blob/0.11.x/CHANGELOG.md
[hyperium/hyper@76fdbcf2]: https://github.com/hyperium/hyper/commit/76fdbcf2
[76fdbcf2]: https://github.com/dekellum/hyperx/commit/76fdbcf23cd35cebb03bf4c0e3025b671578bd75
