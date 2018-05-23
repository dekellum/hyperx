#![doc(html_root_url = "https://docs.rs/hyperx/0.11.27")]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_extern_crates)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! # hyper_x_
//!
//! Hyper is the low-level HTTP implementation for Rust. Hyper_x_ is an
//! e_x_traction of the hyper 0.11 typed header module, with minimized
//! dependencies, for continued use with hyper 0.12 (current master, work in
//! progress), where this module was dropped in preference to the
//! byte-oriented `http::header` module.

extern crate base64;
extern crate bytes;
#[cfg(feature = "compat")]
extern crate http;
extern crate httparse;
extern crate language_tags;
#[macro_use] extern crate log;
pub extern crate mime;
#[macro_use] extern crate percent_encoding;
extern crate time;
extern crate unicase;

#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub use error::{Result, Error};
pub use header::Headers;
pub use method::Method;

mod error;
mod method;
mod common;
pub mod header;
mod uri;

