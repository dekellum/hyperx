#!/usr/bin/bash -ve

cargo -Z minimal-versions generate-lockfile

# from iovec failure due to libc:
# libc v0.2.1
# ├── iovec v0.1.0
# │   └── bytes v0.4.4
# │       ├── http v0.1.0
# │       │   └── hyperx v0.15.2 (/home/david/src/hyperx)
# │       └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
# └── time v0.1.37
# └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
# A fix appears to have never been released:
# https://github.com/carllerche/iovec/commit/b54dfbe145d41385ac8826819b3603ed27cd7612
cargo update -p libc --precise 0.2.3
