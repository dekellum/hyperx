#!/usr/bin/bash -ve

cargo update -p bytes                   --precise 0.4.4
cargo update -p http                    --precise 0.1.0
cargo update -p httparse                --precise 1.0.0

cargo update -p language-tags           --precise 0.2.0
cargo update -p log                     --precise 0.4.0
cargo update -p mime                    --precise 0.3.2
cargo update -p percent-encoding        --precise 1.0.0
cargo update -p time                    --precise 0.1.37
cargo update -p unicase                 --precise 2.0.0
