# This work is now merged into Rust https://github.com/rust-lang/rust/pull/27110

# Optimized (U)Int* format/parse methods

As of Jun 14 2015 Rust still uses a naive implementation of integer to decimal strings. The string parsing can be improved as well.

# Results

## string length histograms for u32

* h (big numbers skew)
[0, 0, 5, 29, 103, 212, 551, 1138, 1887, 3196, 2879]
* m (slight small number skew):
[0, 2807, 1334, 1057, 905, 821, 772, 707, 627, 605, 365]
* l (small numbers skew):
[0, 8004, 567, 351, 248, 212, 170, 126, 136, 112, 74]

Running with rustc 1.3.0-nightly (7ea2674c7 2015-07-13) @ x64 Linux - Intel(R) Core(TM) i7-2670QM @ 2.20Ghz (My notebook CPU)

```
test bench::display_h_new_u08     ... bench:      66,971 ns/iter (+/- 35,906)
test bench::display_h_new_u16     ... bench:     368,922 ns/iter (+/- 46,782)
test bench::display_h_new_u32     ... bench:   4,115,146 ns/iter (+/- 400,042)
test bench::display_h_new_u64     ... bench:   4,934,655 ns/iter (+/- 396,045)
test bench::display_h_stdlib_u08  ... bench:      73,334 ns/iter (+/- 17,527)
test bench::display_h_stdlib_u16  ... bench:     446,509 ns/iter (+/- 121,848)
test bench::display_h_stdlib_u32  ... bench:   5,597,621 ns/iter (+/- 1,597,976)
test bench::display_h_stdlib_u64  ... bench:   8,605,499 ns/iter (+/- 631,995)
test bench::display_l_new_u08     ... bench:      66,258 ns/iter (+/- 5,457)
test bench::display_l_new_u16     ... bench:     367,209 ns/iter (+/- 31,173)
test bench::display_l_new_u32     ... bench:   3,993,238 ns/iter (+/- 358,407)
test bench::display_l_new_u64     ... bench:   5,155,402 ns/iter (+/- 405,838)
test bench::display_l_stdlib_u08  ... bench:      71,479 ns/iter (+/- 5,400)
test bench::display_l_stdlib_u16  ... bench:     423,175 ns/iter (+/- 37,471)
test bench::display_l_stdlib_u32  ... bench:   5,526,352 ns/iter (+/- 528,696)
test bench::display_l_stdlib_u64  ... bench:   8,765,285 ns/iter (+/- 631,288)
test bench::display_m_new_u08     ... bench:      69,366 ns/iter (+/- 2,735)
test bench::display_m_new_u16     ... bench:     393,204 ns/iter (+/- 12,167)
test bench::display_m_new_u32     ... bench:   4,011,073 ns/iter (+/- 372,304)
test bench::display_m_new_u64     ... bench:   4,982,899 ns/iter (+/- 452,298)
test bench::display_m_stdlib_u08  ... bench:      72,506 ns/iter (+/- 6,468)
test bench::display_m_stdlib_u16  ... bench:     424,319 ns/iter (+/- 29,545)
test bench::display_m_stdlib_u32  ... bench:   5,618,650 ns/iter (+/- 235,886)
test bench::display_m_stdlib_u64  ... bench:   8,735,070 ns/iter (+/- 321,531)
test bench::from_str_h_new_u08    ... bench:      28,153 ns/iter (+/- 624)
test bench::from_str_h_new_u16    ... bench:     223,513 ns/iter (+/- 11,554)
test bench::from_str_h_new_u32    ... bench:   3,098,935 ns/iter (+/- 231,022)
test bench::from_str_h_new_u64    ... bench:   5,009,900 ns/iter (+/- 341,961)
test bench::from_str_h_stdlib_u08 ... bench:      34,033 ns/iter (+/- 2,068)
test bench::from_str_h_stdlib_u16 ... bench:     248,785 ns/iter (+/- 14,208)
test bench::from_str_h_stdlib_u32 ... bench:   4,150,536 ns/iter (+/- 266,070)
test bench::from_str_h_stdlib_u64 ... bench:   6,817,997 ns/iter (+/- 449,838)
test bench::from_str_l_new_u08    ... bench:      27,552 ns/iter (+/- 1,500)
test bench::from_str_l_new_u16    ... bench:     234,360 ns/iter (+/- 13,144)
test bench::from_str_l_new_u32    ... bench:   3,140,261 ns/iter (+/- 248,175)
test bench::from_str_l_new_u64    ... bench:   5,176,583 ns/iter (+/- 350,416)
test bench::from_str_l_stdlib_u08 ... bench:      35,060 ns/iter (+/- 2,154)
test bench::from_str_l_stdlib_u16 ... bench:     252,135 ns/iter (+/- 23,461)
test bench::from_str_l_stdlib_u32 ... bench:   4,154,599 ns/iter (+/- 369,606)
test bench::from_str_l_stdlib_u64 ... bench:   6,892,767 ns/iter (+/- 213,030)
test bench::from_str_m_new_u08    ... bench:      28,252 ns/iter (+/- 1,384)
test bench::from_str_m_new_u16    ... bench:     231,051 ns/iter (+/- 16,540)
test bench::from_str_m_new_u32    ... bench:   3,166,504 ns/iter (+/- 134,418)
test bench::from_str_m_new_u64    ... bench:   5,103,195 ns/iter (+/- 218,912)
test bench::from_str_m_stdlib_u08 ... bench:      35,012 ns/iter (+/- 2,735)
test bench::from_str_m_stdlib_u16 ... bench:     250,967 ns/iter (+/- 14,708)
test bench::from_str_m_stdlib_u32 ... bench:   4,101,845 ns/iter (+/- 205,802)
test bench::from_str_m_stdlib_u64 ... bench:   6,823,001 ns/iter (+/- 267,215)
```

Running with rustc 1.3.0-nightly (e4e93196e 2015-07-14) @ x86 Linux - Intel(R) Xeon(R) CPU E5-2630L v2 @ 2.40GHz

```
test bench::display_h_new_u08     ... bench:      94,246 ns/iter (+/- 34,872)
test bench::display_h_new_u16     ... bench:     533,805 ns/iter (+/- 22,499)
test bench::display_h_new_u32     ... bench:   6,127,747 ns/iter (+/- 2,192,789)
test bench::display_h_new_u64     ... bench:  14,994,203 ns/iter (+/- 1,609,345)
test bench::display_h_stdlib_u08  ... bench:     107,233 ns/iter (+/- 8,571)
test bench::display_h_stdlib_u16  ... bench:     631,186 ns/iter (+/- 11,332)
test bench::display_h_stdlib_u32  ... bench:   7,696,344 ns/iter (+/- 957,917)
test bench::display_h_stdlib_u64  ... bench:  45,677,401 ns/iter (+/- 4,991,344)
test bench::display_l_new_u08     ... bench:      95,855 ns/iter (+/- 27,735)
test bench::display_l_new_u16     ... bench:     532,084 ns/iter (+/- 40,479)
test bench::display_l_new_u32     ... bench:   5,973,953 ns/iter (+/- 211,676)
test bench::display_l_new_u64     ... bench:  14,773,064 ns/iter (+/- 1,276,579)
test bench::display_l_stdlib_u08  ... bench:     106,350 ns/iter (+/- 63,963)
test bench::display_l_stdlib_u16  ... bench:     637,746 ns/iter (+/- 101,005)
test bench::display_l_stdlib_u32  ... bench:   7,740,640 ns/iter (+/- 848,478)
test bench::display_l_stdlib_u64  ... bench:  44,846,932 ns/iter (+/- 4,514,694)
test bench::display_m_new_u08     ... bench:      94,549 ns/iter (+/- 13,029)
test bench::display_m_new_u16     ... bench:     546,030 ns/iter (+/- 35,804)
test bench::display_m_new_u32     ... bench:   5,983,924 ns/iter (+/- 1,180,559)
test bench::display_m_new_u64     ... bench:  14,817,873 ns/iter (+/- 2,271,464)
test bench::display_m_stdlib_u08  ... bench:     107,806 ns/iter (+/- 8,805)
test bench::display_m_stdlib_u16  ... bench:     630,714 ns/iter (+/- 6,586)
test bench::display_m_stdlib_u32  ... bench:   7,784,210 ns/iter (+/- 358,601)
test bench::display_m_stdlib_u64  ... bench:  46,223,927 ns/iter (+/- 6,553,176)
test bench::from_str_h_new_u08    ... bench:      23,682 ns/iter (+/- 3,590)
test bench::from_str_h_new_u16    ... bench:     190,916 ns/iter (+/- 29,688)
test bench::from_str_h_new_u32    ... bench:   2,649,952 ns/iter (+/- 308,576)
test bench::from_str_h_new_u64    ... bench:  23,458,434 ns/iter (+/- 2,327,427)
test bench::from_str_h_stdlib_u08 ... bench:      45,551 ns/iter (+/- 6,968)
test bench::from_str_h_stdlib_u16 ... bench:     313,739 ns/iter (+/- 17,175)
test bench::from_str_h_stdlib_u32 ... bench:   4,615,669 ns/iter (+/- 470,766)
test bench::from_str_h_stdlib_u64 ... bench:  30,589,482 ns/iter (+/- 2,278,996)
test bench::from_str_l_new_u08    ... bench:      23,763 ns/iter (+/- 5,545)
test bench::from_str_l_new_u16    ... bench:     185,472 ns/iter (+/- 33,097)
test bench::from_str_l_new_u32    ... bench:   2,691,307 ns/iter (+/- 473,886)
test bench::from_str_l_new_u64    ... bench:  22,952,593 ns/iter (+/- 1,963,742)
test bench::from_str_l_stdlib_u08 ... bench:      45,285 ns/iter (+/- 16,337)
test bench::from_str_l_stdlib_u16 ... bench:     313,624 ns/iter (+/- 6,643)
test bench::from_str_l_stdlib_u32 ... bench:   4,595,679 ns/iter (+/- 1,876,361)
test bench::from_str_l_stdlib_u64 ... bench:  30,434,683 ns/iter (+/- 1,901,996)
test bench::from_str_m_new_u08    ... bench:      23,812 ns/iter (+/- 1,505)
test bench::from_str_m_new_u16    ... bench:     185,553 ns/iter (+/- 19,788)
test bench::from_str_m_new_u32    ... bench:   2,614,920 ns/iter (+/- 66,230)
test bench::from_str_m_new_u64    ... bench:  23,241,778 ns/iter (+/- 3,474,077)
test bench::from_str_m_stdlib_u08 ... bench:      45,634 ns/iter (+/- 1,436)
test bench::from_str_m_stdlib_u16 ... bench:     316,479 ns/iter (+/- 21,212)
test bench::from_str_m_stdlib_u32 ... bench:   4,609,147 ns/iter (+/- 487,068)
test bench::from_str_m_stdlib_u64 ... bench:  30,165,173 ns/iter (+/- 1,601,830)
```
