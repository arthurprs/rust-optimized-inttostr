# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

Running with rust 1.2 nightly @ x64 Linux - Intel(R) Core(TM) i7-2670QM CPU @ 2.20GHz

* random: random numbers distributed equally in the type range
* skewed: random numbers skewed to smaller numbers (trying to reflect real usage)

```
test bench::random_new_u08     ... bench:     73063 ns/iter (+/- 2712)
test bench::random_new_u16     ... bench:    349182 ns/iter (+/- 16315)
test bench::random_new_u32     ... bench:   3925660 ns/iter (+/- 342846)
test bench::random_new_u64     ... bench:   5347440 ns/iter (+/- 261020)
test bench::random_stdlib_u08  ... bench:     80722 ns/iter (+/- 3867)
test bench::random_stdlib_u16  ... bench:    385708 ns/iter (+/- 30096)
test bench::random_stdlib_u32  ... bench:   4446318 ns/iter (+/- 248931)
test bench::random_stdlib_u64  ... bench:   6747984 ns/iter (+/- 403998)
test bench::random_strconv_u08 ... bench:     80608 ns/iter (+/- 6822)
test bench::random_strconv_u16 ... bench:    393144 ns/iter (+/- 23496)
test bench::random_strconv_u32 ... bench:   4427777 ns/iter (+/- 285315)
test bench::random_strconv_u64 ... bench:   6761469 ns/iter (+/- 485563)
test bench::skewed_new_u08     ... bench:     68073 ns/iter (+/- 3216)
test bench::skewed_new_u16     ... bench:    349168 ns/iter (+/- 16996)
test bench::skewed_new_u32     ... bench:   3573622 ns/iter (+/- 274548)
test bench::skewed_new_u64     ... bench:   3675003 ns/iter (+/- 228779)
test bench::skewed_stdlib_u08  ... bench:     81506 ns/iter (+/- 4399)
test bench::skewed_stdlib_u16  ... bench:    440257 ns/iter (+/- 25390)
test bench::skewed_stdlib_u32  ... bench:   4772374 ns/iter (+/- 293192)
test bench::skewed_stdlib_u64  ... bench:   5878061 ns/iter (+/- 306407)
test bench::skewed_strconv_u08 ... bench:     82853 ns/iter (+/- 3512)
test bench::skewed_strconv_u16 ... bench:    431086 ns/iter (+/- 24820)
test bench::skewed_strconv_u32 ... bench:   4656460 ns/iter (+/- 313516)
test bench::skewed_strconv_u64 ... bench:   5851998 ns/iter (+/- 250406)
```

# TODO

* Check performance for 32bit systems
* Further optimizations
* Possibly adapt the code and file a PR to rust stdlib?
