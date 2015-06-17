# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

Running with rust 1.2 nightly @ x64 Linux - Intel(R) Core(TM) i7-2670QM CPU @ 2.20GHz

* random: random numbers distributed equally in the type range
* skewed: random numbers skewed to smaller numbers (trying to reflect real usage)

```
test bench::random_new_u08     ... bench:     76575 ns/iter (+/- 3333)
test bench::random_new_u16     ... bench:    386958 ns/iter (+/- 32014)
test bench::random_new_u32     ... bench:   4191486 ns/iter (+/- 464866)
test bench::random_new_u64     ... bench:   5527040 ns/iter (+/- 409646)
test bench::random_stdlib_u08  ... bench:     85138 ns/iter (+/- 4507)
test bench::random_stdlib_u16  ... bench:    393569 ns/iter (+/- 25723)
test bench::random_stdlib_u32  ... bench:   4610918 ns/iter (+/- 171989)
test bench::random_stdlib_u64  ... bench:   6785896 ns/iter (+/- 482693)
test bench::random_strconv_u08 ... bench:     86210 ns/iter (+/- 5052)
test bench::random_strconv_u16 ... bench:    391152 ns/iter (+/- 18715)
test bench::random_strconv_u32 ... bench:   4517918 ns/iter (+/- 226041)
test bench::random_strconv_u64 ... bench:   6595524 ns/iter (+/- 575203)
test bench::skewed_new_u08     ... bench:     70107 ns/iter (+/- 4295)
test bench::skewed_new_u16     ... bench:    375911 ns/iter (+/- 32367)
test bench::skewed_new_u32     ... bench:   4408632 ns/iter (+/- 177293)
test bench::skewed_new_u64     ... bench:   5185259 ns/iter (+/- 233081)
test bench::skewed_stdlib_u08  ... bench:     77604 ns/iter (+/- 2557)
test bench::skewed_stdlib_u16  ... bench:    432629 ns/iter (+/- 24703)
test bench::skewed_stdlib_u32  ... bench:   5285474 ns/iter (+/- 262226)
test bench::skewed_stdlib_u64  ... bench:   6496465 ns/iter (+/- 225418)
test bench::skewed_strconv_u08 ... bench:     79549 ns/iter (+/- 2836)
test bench::skewed_strconv_u16 ... bench:    433645 ns/iter (+/- 19394)
test bench::skewed_strconv_u32 ... bench:   5243582 ns/iter (+/- 285716)
test bench::skewed_strconv_u64 ... bench:   6464695 ns/iter (+/- 240468)
```

# TODO

* Check performance for 32bit systems
* Further optimizations
* Possibly adapt the code and file a PR to rust stdlib?
