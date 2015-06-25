# Optimized (U)Int* format methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings.

My hopes are that this work allows measurable improvements to json serializers.

# Results

## string length histograms for u32

* h (big numbers skew)
[0, 0, 5, 29, 103, 212, 551, 1138, 1887, 3196, 2879]
* m (slight small number skew):
[0, 1505, 1177, 1062, 1040, 983, 952, 955, 919, 877, 530]
* l (small numbers skew):
[0, 4684, 1205, 863, 713, 600, 501, 439, 416, 346, 233]

Running with rustc 1.2.0-nightly (cffaf0e7a 2015-06-23) @ x64 Linux - Intel(R) Core(TM) i7-2670QM @ 2.20Ghz (My notebook CPU)

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

Running with rustc 1.3.0-nightly (e5a28bca7 2015-06-25) @ x86 Linux - Intel(R) Xeon(R) CPU E5-2650 0 @ 2.00GHz (EC2 c1.medium)

```
test bench::skewed_h_new_u08    ... bench:     128,062 ns/iter (+/- 624)
test bench::skewed_h_new_u16    ... bench:     701,687 ns/iter (+/- 2,595)
test bench::skewed_h_new_u32    ... bench:   8,013,071 ns/iter (+/- 86,295)
test bench::skewed_h_new_u64    ... bench:  20,619,636 ns/iter (+/- 244,472)
test bench::skewed_h_stdlib_u08 ... bench:     139,061 ns/iter (+/- 4,208)
test bench::skewed_h_stdlib_u16 ... bench:     840,872 ns/iter (+/- 8,870)
test bench::skewed_h_stdlib_u32 ... bench:  10,934,092 ns/iter (+/- 86,377)
test bench::skewed_h_stdlib_u64 ... bench:  62,690,245 ns/iter (+/- 4,648,790)
test bench::skewed_l_new_u08    ... bench:     128,245 ns/iter (+/- 1,491)
test bench::skewed_l_new_u16    ... bench:     702,062 ns/iter (+/- 13,180)
test bench::skewed_l_new_u32    ... bench:   8,021,507 ns/iter (+/- 325,452)
test bench::skewed_l_new_u64    ... bench:  20,596,010 ns/iter (+/- 962,453)
test bench::skewed_l_stdlib_u08 ... bench:     139,014 ns/iter (+/- 7,428)
test bench::skewed_l_stdlib_u16 ... bench:     840,780 ns/iter (+/- 16,955)
test bench::skewed_l_stdlib_u32 ... bench:  10,926,288 ns/iter (+/- 309,821)
test bench::skewed_l_stdlib_u64 ... bench:  62,649,913 ns/iter (+/- 1,106,527)
test bench::skewed_m_new_u08    ... bench:     128,949 ns/iter (+/- 16,267)
test bench::skewed_m_new_u16    ... bench:     706,043 ns/iter (+/- 73,190)
test bench::skewed_m_new_u32    ... bench:   8,001,205 ns/iter (+/- 219,644)
test bench::skewed_m_new_u64    ... bench:  20,569,162 ns/iter (+/- 430,049)
test bench::skewed_m_stdlib_u08 ... bench:     138,840 ns/iter (+/- 5,948)
test bench::skewed_m_stdlib_u16 ... bench:     840,655 ns/iter (+/- 9,596)
test bench::skewed_m_stdlib_u32 ... bench:  10,949,664 ns/iter (+/- 191,620)
test bench::skewed_m_stdlib_u64 ... bench:  62,858,086 ns/iter (+/- 1,316,625)
```

# TODO

* Possibly adapt the code and file a PR to rust stdlib?
