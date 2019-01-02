## 0.13.2 (TBD)

## 0.13.1 (2018-6-26)

* Remove `error::UriError` re-export and `error::Canceled` which are unused
  internally and where not exported from this crate.  (dekellum #5)

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
