#!/usr/bin/bash -ve

cargo -Z minimal-versions generate-lockfile
cargo update -p unicase --precise 2.1.0
cargo update -p libc --precise 0.2.3
