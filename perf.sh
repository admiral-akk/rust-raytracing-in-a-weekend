#!/bin/sh
rm -rf ./perf
mkdir perf
cargo build --profile release
/home/karpierz/WSL2-Linux-Kernel/tools/perf/perf record -g target/release/rust-ray
/home/karpierz/WSL2-Linux-Kernel/tools/perf/perf report --hierarchy >> perf.txt
rustfilt -i perf.txt -o demangled.txt
mv demangled.txt ./perf/demangled.txt
rm perf.txt