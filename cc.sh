#!/bin/bash
cargo build --release
cbindgen --config cbindgen.toml --crate space_net --output src/c_ffi/space_net.h --lang c
mkdir bin
gcc examples/cexample_boot.c -o bin/cexample_boot -Ltarget/release -lspace_net -Isrc/c_ffi
gcc examples/cexample_node.c -o bin/cexample_node -Ltarget/release -lspace_net -Isrc/c_ffi
gcc examples/cexample_node_sub.c -o bin/cexample_node_sub -Ltarget/release -lspace_net -Isrc/c_ffi
gcc examples/cexample_node_pub.c -o bin/cexample_node_pub -Ltarget/release -lspace_net -Isrc/c_ffi
export LD_LIBRARY_PATH=target/release
