# benchmarking `fast_parse`

```
➤ cargo bench
   Compiling int-bench v0.1.0 (/Users/samuel/code/pydantic-core/int-bench)
    Finished bench [optimized] target(s) in 0.73s
     Running unittests src/main.rs (target/release/deps/int_bench-ae52f29fc1314e8d)

running 3 tests
test tests::test_fast_parse ... ignored
test tests::bench_fast_parse ... bench:          17 ns/iter (+/- 0)
test tests::bench_std_parse  ... bench:          30 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 1 ignored; 2 measured; 0 filtered out; finished in 0.53s

➤ rustc --version
rustc 1.63.0-nightly (e09449220 2022-05-31)
```
