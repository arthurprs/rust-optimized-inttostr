# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

Running with rustc 1.2.0-nightly (cffaf0e7a 2015-06-23) @ x64 Linux - Intel(R) Core(TM) i7-2670QM CPU @ 2.20GHz

* test string length histograms for u32
* h (big numbers skew)
* [0, 0, 5, 29, 103, 212, 551, 1138, 1887, 3196, 2879]
* m (slight small number skew) :
* [0, 1505, 1177, 1062, 1040, 983, 952, 955, 919, 877, 530]
* l (small numbers skew):
* [0, 3547, 1317, 1000, 827, 730, 665, 582, 538, 482, 312]

```
test bench::skewed_h_new_u08    ... bench:      70,989 ns/iter (+/- 7,817)
test bench::skewed_h_new_u16    ... bench:     378,118 ns/iter (+/- 22,320)
test bench::skewed_h_new_u32    ... bench:   4,290,177 ns/iter (+/- 420,060)
test bench::skewed_h_new_u64    ... bench:   5,159,141 ns/iter (+/- 509,104)
test bench::skewed_h_stdlib_u08 ... bench:      70,296 ns/iter (+/- 5,432)
test bench::skewed_h_stdlib_u16 ... bench:     424,123 ns/iter (+/- 29,182)
test bench::skewed_h_stdlib_u32 ... bench:   5,462,581 ns/iter (+/- 490,533)
test bench::skewed_h_stdlib_u64 ... bench:   8,467,202 ns/iter (+/- 800,586)
test bench::skewed_l_new_u08    ... bench:      70,563 ns/iter (+/- 5,619)
test bench::skewed_l_new_u16    ... bench:     380,621 ns/iter (+/- 32,223)
test bench::skewed_l_new_u32    ... bench:   4,224,106 ns/iter (+/- 197,825)
test bench::skewed_l_new_u64    ... bench:   5,145,698 ns/iter (+/- 505,134)
test bench::skewed_l_stdlib_u08 ... bench:      70,344 ns/iter (+/- 5,702)
test bench::skewed_l_stdlib_u16 ... bench:     423,822 ns/iter (+/- 16,516)
test bench::skewed_l_stdlib_u32 ... bench:   5,551,774 ns/iter (+/- 509,895)
test bench::skewed_l_stdlib_u64 ... bench:   8,196,451 ns/iter (+/- 623,838)
test bench::skewed_m_new_u08    ... bench:      68,856 ns/iter (+/- 4,890)
test bench::skewed_m_new_u16    ... bench:     376,159 ns/iter (+/- 40,527)
test bench::skewed_m_new_u32    ... bench:   4,274,610 ns/iter (+/- 223,194)
test bench::skewed_m_new_u64    ... bench:   5,115,321 ns/iter (+/- 258,833)
test bench::skewed_m_stdlib_u08 ... bench:      68,915 ns/iter (+/- 3,567)
test bench::skewed_m_stdlib_u16 ... bench:     416,575 ns/iter (+/- 16,627)
test bench::skewed_m_stdlib_u32 ... bench:   5,299,133 ns/iter (+/- 198,394)
test bench::skewed_m_stdlib_u64 ... bench:   8,326,535 ns/iter (+/- 299,953)

```

# TODO

* Check performance for 32bit systems
* Further optimizations
* Possibly adapt the code and file a PR to rust stdlib?
