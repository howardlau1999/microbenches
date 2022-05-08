Some micro-benchmarks.

Run `cargo bench` to see the results.

Find assembly code at `target/release/deps/benchmarks-<hash>.s`.

Benchmarks:

`div`: Compare integer division and floating point division. (Floating point division is faster!)

`simd_sum`: Sum an array of numbers using SIMD.

`mlp`: Pointer chasing for testing memory latency. (Numbers are the time used by 200 memory loads.)