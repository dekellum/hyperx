# hyper*x*

[![crates.io](https://img.shields.io/crates/v/hyperx.svg?maxAge=3600)](https://crates.io/crates/hyperx)
[![Rustdoc](https://docs.rs/hyperx/badge.svg)](https://docs.rs/hyperx)
[![Travis CI Build](https://travis-ci.org/dekellum/hyperx.svg?branch=master)](https://travis-ci.org/dekellum/hyperx)
[![Appveyor CI Build](https://ci.appveyor.com/api/projects/status/99slabo810em9xvy?svg=true)](https://ci.appveyor.com/project/dekellum/hyperx)
[![deps status](https://deps.rs/repo/github/dekellum/hyperx/status.svg)](https://deps.rs/repo/github/dekellum/hyperx)

[Hyper] is the low-level HTTP implementation for Rust. Hyper*x* is an
e*x*traction of the hyper 0.11 typed header module, with minimized
dependencies, for continued use with hyper 0.12 or later,
where this module was dropped in preference to the byte-oriented
`http::header` module.

[Hyper]: https://github.com/hyperium/hyper

## Minimum supported rust version

MSRV := 1.39.0

The crate will fail fast on any lower rustc (via a build.rs version
check) and is also CI tested on this version.

## License

The MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
