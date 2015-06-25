# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

Running with rustc 1.2.0-nightly (cffaf0e7a 2015-06-23) @ x64 Linux - Intel(R) Core(TM) i7-2670QM CPU @ 2.20GHz

## string length histograms for u32

Ex: "3" => 1, "31" => 2, "1001" => 4 

* h (big numbers skew)
[0, 0, 5, 29, 103, 212, 551, 1138, 1887, 3196, 2879]
* m (slight small number skew):
[0, 1505, 1177, 1062, 1040, 983, 952, 955, 919, 877, 530]
* l (small numbers skew):
[0, 4684, 1205, 863, 713, 600, 501, 439, 416, 346, 233]

```
test bench::skewed_h_new_u08    ... bench:      67,156 ns/iter (+/- 3,687)
test bench::skewed_h_new_u16    ... bench:     376,573 ns/iter (+/- 23,732)
test bench::skewed_h_new_u32    ... bench:   4,202,419 ns/iter (+/- 203,200)
test bench::skewed_h_new_u64    ... bench:   5,097,971 ns/iter (+/- 337,608)
test bench::skewed_h_stdlib_u08 ... bench:      69,270 ns/iter (+/- 3,321)
test bench::skewed_h_stdlib_u16 ... bench:     420,660 ns/iter (+/- 20,196)
test bench::skewed_h_stdlib_u32 ... bench:   5,451,519 ns/iter (+/- 417,856)
test bench::skewed_h_stdlib_u64 ... bench:   8,360,505 ns/iter (+/- 453,566)
test bench::skewed_l_new_u08    ... bench:      68,705 ns/iter (+/- 3,657)
test bench::skewed_l_new_u16    ... bench:     376,786 ns/iter (+/- 20,804)
test bench::skewed_l_new_u32    ... bench:   4,207,858 ns/iter (+/- 210,143)
test bench::skewed_l_new_u64    ... bench:   5,117,710 ns/iter (+/- 350,017)
test bench::skewed_l_stdlib_u08 ... bench:      68,252 ns/iter (+/- 4,251)
test bench::skewed_l_stdlib_u16 ... bench:     417,692 ns/iter (+/- 22,882)
test bench::skewed_l_stdlib_u32 ... bench:   5,383,473 ns/iter (+/- 322,479)
test bench::skewed_l_stdlib_u64 ... bench:   8,380,458 ns/iter (+/- 357,894)
test bench::skewed_m_new_u08    ... bench:      68,585 ns/iter (+/- 3,099)
test bench::skewed_m_new_u16    ... bench:     375,588 ns/iter (+/- 20,074)
test bench::skewed_m_new_u32    ... bench:   4,178,506 ns/iter (+/- 234,058)
test bench::skewed_m_new_u64    ... bench:   5,041,359 ns/iter (+/- 302,088)
test bench::skewed_m_stdlib_u08 ... bench:      67,975 ns/iter (+/- 3,570)
test bench::skewed_m_stdlib_u16 ... bench:     421,650 ns/iter (+/- 22,984)
test bench::skewed_m_stdlib_u32 ... bench:   5,422,042 ns/iter (+/- 330,263)
test bench::skewed_m_stdlib_u64 ... bench:   8,358,590 ns/iter (+/- 418,777)
```

# TODO

* Check performance for 32bit systems
* Further optimizations
* Possibly adapt the code and file a PR to rust stdlib?
