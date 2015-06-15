# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

Running with rust 1.2 nightly @ x64 Linux - Intel(R) Core(TM) i7-2670QM CPU @ 2.20GHz


```
test tests::_warmup   ... bench:         0 ns/iter (+/- 0)
test tests::new_08    ... bench:       562 ns/iter (+/- 111)
test tests::new_16    ... bench:      1424 ns/iter (+/- 60)
test tests::new_32    ... bench:      3342 ns/iter (+/- 92)
test tests::new_64    ... bench:      7692 ns/iter (+/- 373)
test tests::stdlib_08 ... bench:       626 ns/iter (+/- 12)
test tests::stdlib_16 ... bench:      1540 ns/iter (+/- 113)
test tests::stdlib_32 ... bench:      3887 ns/iter (+/- 72)
test tests::stdlib_64 ... bench:     11436 ns/iter (+/- 317)
```

# TODO

* Check performance for 32bit systems
* Further optimizations
* Possibly adapt the code and file a PR to rust stdlib?
