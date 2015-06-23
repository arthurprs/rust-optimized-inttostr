

// use strconv::int2dec::UintToDec;
use Wrapper;
use std::fmt::Write as fmtWrite;
use std::io::{self, Write};
use test;
use rand::{Rng, SeedableRng, StdRng};
 
const SEED: &'static [usize] = &[0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

struct VoidWriter;


impl Write for VoidWriter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    	test::black_box(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


macro_rules! bench_new {
	($t:ty: $name:ident, $samples:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<_> = (0..$samples).map(|_| Wrapper(rng.gen::<$t>())).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
}

// macro_rules! bench_strconv {
// 	($t:ty: $name:ident, $samples:expr) => {
// 		#[bench]
// 		fn $name(b: &mut test::Bencher) {
// 			let mut rng: StdRng = SeedableRng::from_seed(SEED);
// 			let mut buf: VoidWriter = VoidWriter;
// 			let items: Vec<_> = (0..$samples).map(|_| UintToDec(rng.gen::<$t>())).collect();
// 			b.iter(|| {
// 				for i in &items {
// 					test::black_box(buf.write_fmt(format_args!("{}", *i)));
// 				}
// 			});
// 		}
// 	};
// }

macro_rules! bench_stdlib {
	($t:ty: $name:ident, $samples:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<$t> = (0..$samples).map(|_| rng.gen::<$t>()).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
}

macro_rules! bench_new_skewed {
	($t:ty: $name:ident, $skew:expr, $samples:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<_> = (0..$samples).map(|_| {
				let x: f64 = rng.gen();
				let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(1f64);
				let x = (x * (u64::max_value() as f64).log10()) as u64;
				let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
				Wrapper(x as $t)
			}).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
	($t:ty: $name:ident, $samples:expr) => {
		bench_new_skewed!($t: $name, 25f64, $samples);
	}
}

// macro_rules! bench_strconv_skewed {
// 	($t:ty: $name:ident, $skew:expr, $samples:expr) => {
// 		#[bench]
// 		fn $name(b: &mut test::Bencher) {
// 			let mut rng: StdRng = SeedableRng::from_seed(SEED);
// 			let mut buf: VoidWriter = VoidWriter;
// 			let items: Vec<_> = (0..$samples).map(|_| {
// 				let x: f64 = rng.gen();
// 				let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(1f64);
// 				let x = (x * (u64::max_value() as f64).log10()) as u64;
// 				let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
// 				UintToDec(x as $t )
// 			}).collect();
// 			b.iter(|| {
// 				for i in &items {
// 					test::black_box(buf.write_fmt(format_args!("{}", *i)));
// 				}
// 			});
// 		}
// 	};
// 	($t:ty: $name:ident, $samples:expr) => {
// 		bench_strconv_skewed!($t: $name, 25f64, $samples);
// 	}
// }

macro_rules! bench_stdlib_skewed {
	($t:ty: $name:ident, $skew:expr, $samples:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<_> = (0..$samples).map(|_| {
				let x: f64 = rng.gen();
				let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(1f64);
				let x = (x * (u64::max_value() as f64).log10()) as u64;
				let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
				x as $t
			}).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
	($t:ty: $name:ident, $samples:expr) => {
		bench_stdlib_skewed!($t: $name, 25f64, $samples);
	}
}

bench_new!(u8: random_new_u08, 2000);
bench_new!(u16: random_new_u16, 10000);
bench_new!(u32: random_new_u32, 100000);
bench_new!(u64: random_new_u64, 100000);

// bench_strconv!(u8: random_strconv_u08, 2000);
// bench_strconv!(u16: random_strconv_u16, 10000);
// bench_strconv!(u32: random_strconv_u32, 100000);
// bench_strconv!(u64: random_strconv_u64, 100000);

bench_stdlib_skewed!(u8: random_stdlib_u08, 2000);
bench_stdlib_skewed!(u16: random_stdlib_u16, 10000);
bench_stdlib_skewed!(u32: random_stdlib_u32, 100000);
bench_stdlib_skewed!(u64: random_stdlib_u64, 100000);

bench_new_skewed!(u8: skewed_new_u08, 2000);
bench_new_skewed!(u16: skewed_new_u16, 10000);
bench_new_skewed!(u32: skewed_new_u32, 100000);
bench_new_skewed!(u64: skewed_new_u64, 100000);

// bench_strconv_skewed!(u8: skewed_strconv_u08, 2000);
// bench_strconv_skewed!(u16: skewed_strconv_u16, 10000);
// bench_strconv_skewed!(u32: skewed_strconv_u32, 100000);
// bench_strconv_skewed!(u64: skewed_strconv_u64, 100000);

bench_stdlib_skewed!(u8: skewed_stdlib_u08, 2000);
bench_stdlib_skewed!(u16: skewed_stdlib_u16, 10000);
bench_stdlib_skewed!(u32: skewed_stdlib_u32, 100000);
bench_stdlib_skewed!(u64: skewed_stdlib_u64, 100000);