# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

Running with rustc 1.2.0-nightly (cffaf0e7a 2015-06-23) @ x64 Linux - Intel(R) Core(TM) i7-2670QM CPU @ 2.20GHz

## string length histograms for u32

Ex: "3" => 1, "31" => 2, "1001" => 4 

* h (big numbers skew)
[0, 0, 5, 29, 103, 212, 551, 1138, 1887, 3196, 2879]
* m (slight small number skew) :
[0, 1505, 1177, 1062, 1040, 983, 952, 955, 919, 877, 530]
* l (small numbers skew):
[0, 3547, 1317, 1000, 827, 730, 665, 582, 538, 482, 312]

```
test bench::skewed_h_new_u08    ... bench:      64,249 ns/iter (+/- 5,755)
test bench::skewed_h_new_u16    ... bench:     355,169 ns/iter (+/- 40,242)
test bench::skewed_h_new_u32    ... bench:   3,980,737 ns/iter (+/- 309,389)
test bench::skewed_h_new_u64    ... bench:   4,957,603 ns/iter (+/- 305,314)
test bench::skewed_h_stdlib_u08 ... bench:      66,205 ns/iter (+/- 8,166)
test bench::skewed_h_stdlib_u16 ... bench:     413,457 ns/iter (+/- 33,285)
test bench::skewed_h_stdlib_u32 ... bench:   5,239,901 ns/iter (+/- 524,692)
test bench::skewed_h_stdlib_u64 ... bench:   8,210,097 ns/iter (+/- 579,323)
test bench::skewed_l_new_u08    ... bench:      67,127 ns/iter (+/- 4,967)
test bench::skewed_l_new_u16    ... bench:     363,413 ns/iter (+/- 13,125)
test bench::skewed_l_new_u32    ... bench:   4,115,363 ns/iter (+/- 397,448)
test bench::skewed_l_new_u64    ... bench:   4,975,062 ns/iter (+/- 341,895)
test bench::skewed_l_stdlib_u08 ... bench:      66,628 ns/iter (+/- 5,250)
test bench::skewed_l_stdlib_u16 ... bench:     411,865 ns/iter (+/- 42,441)
test bench::skewed_l_stdlib_u32 ... bench:   5,268,278 ns/iter (+/- 361,714)
test bench::skewed_l_stdlib_u64 ... bench:   8,106,945 ns/iter (+/- 380,846)
test bench::skewed_m_new_u08    ... bench:      67,008 ns/iter (+/- 4,101)
test bench::skewed_m_new_u16    ... bench:     359,801 ns/iter (+/- 16,580)
test bench::skewed_m_new_u32    ... bench:   3,999,694 ns/iter (+/- 405,398)
test bench::skewed_m_new_u64    ... bench:   4,853,385 ns/iter (+/- 482,525)
test bench::skewed_m_stdlib_u08 ... bench:      66,191 ns/iter (+/- 7,005)
test bench::skewed_m_stdlib_u16 ... bench:     394,943 ns/iter (+/- 23,838)
test bench::skewed_m_stdlib_u32 ... bench:   5,238,142 ns/iter (+/- 338,026)
test bench::skewed_m_stdlib_u64 ... bench:   8,077,920 ns/iter (+/- 808,231)

```

# TODO

* Check performance for 32bit systems
* Further optimizations
* Possibly adapt the code and file a PR to rust stdlib?
