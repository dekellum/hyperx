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

# due to (fails on windows appveyor):
# winapi v0.2.4
# ├── iovec v0.1.0
# │   └── bytes v0.4.4
# │       ├── http v0.1.0
# │       │   └── hyperx v0.15.2 (/home/david/src/hyperx)
# │       └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
# ├── kernel32-sys v0.2.0
# │   └── time v0.1.37
# │       └── hyperx v0.15.2 (/home/david/src/hyperx) (*)
# └── time v0.1.37 (*)
# An upgrade to winapi (0.3, in 2017!) was never released:
# https://github.com/carllerche/iovec/commit/b90b433f58fb8d64ad6c67d8080cf3da1fce3543
cargo update -p winapi --precise 0.2.8
