#!/usr/bin/bash -ve

cargo -Z minimal-versions generate-lockfile

# due to:
# semver v0.1.0
# └── rustc_version v0.1.0
#     [build-dependencies]
#     └── unicase v2.0.0
#         ├── hyperx v0.15.2 (/home/david/src/hyperx)
#         └── mime v0.3.2
#             └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
# fixed directly, but never released
# https://github.com/hyperium/mime/commit/c9d43b13c4a74b1168a268bd1e241d754992c15e
cargo update -p unicase --precise 2.1.0

# from iovec failure due to libc:
# libc v0.2.1
# ├── iovec v0.1.0
# │   └── bytes v0.4.4
# │       ├── http v0.1.0
# │       │   └── hyperx v0.15.2 (/home/david/src/hyperx)
# │       └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
# └── time v0.1.37
# └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
cargo update -p libc --precise 0.2.3
