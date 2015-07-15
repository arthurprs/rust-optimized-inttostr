use Wrapper;
use std::str::FromStr;
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

macro_rules! bench_new_skewed {
	($t:ty: $name:ident, $samples:expr, $skew:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<Wrapper<$t>> = (0..$samples).map(|_| {
				let x: f64 = rng.gen();
				let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(0.2f64);
				let x = (x * (<$t>::max_value() as f64).log10()) as u32;
				let x = 10u64.pow(x as u32);
				let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
				Wrapper(x as $t)
			}).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(write!(buf, "{}", *i));
				}
			});
		}
	};
}

macro_rules! bench_stdlib_skewed {
	($t:ty: $name:ident, $samples:expr, $skew:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<$t> = (0..$samples).map(|_| {
				let x: f64 = rng.gen();
				let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(0.2f64);
				let x = (x * (<$t>::max_value() as f64).log10()) as u32;
				let x = 10u64.pow(x as u32);
				let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
				x as $t
			}).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(write!(buf, "{}", *i));
				}
			});
		}
	};
}

bench_new_skewed!(u8: skewed_l_new_u08, 2000, 1.5f64);
bench_new_skewed!(u16: skewed_l_new_u16, 10000, 1.5f64);
bench_new_skewed!(u32: skewed_l_new_u32, 100000, 1.5f64);
bench_new_skewed!(u64: skewed_l_new_u64, 100000, 1.5f64);

bench_stdlib_skewed!(u8: skewed_l_stdlib_u08, 2000, 1.5f64);
bench_stdlib_skewed!(u16: skewed_l_stdlib_u16, 10000, 1.5f64);
bench_stdlib_skewed!(u32: skewed_l_stdlib_u32, 100000, 1.5f64);
bench_stdlib_skewed!(u64: skewed_l_stdlib_u64, 100000, 1.5f64);

bench_new_skewed!(u8: skewed_m_new_u08, 2000, 0.6f64);
bench_new_skewed!(u16: skewed_m_new_u16, 10000, 0.6f64);
bench_new_skewed!(u32: skewed_m_new_u32, 100000, 0.6f64);
bench_new_skewed!(u64: skewed_m_new_u64, 100000, 0.6f64);

bench_stdlib_skewed!(u8: skewed_m_stdlib_u08, 2000, 0.6f64);
bench_stdlib_skewed!(u16: skewed_m_stdlib_u16, 10000, 0.6f64);
bench_stdlib_skewed!(u32: skewed_m_stdlib_u32, 100000, 0.6f64);
bench_stdlib_skewed!(u64: skewed_m_stdlib_u64, 100000, 0.6f64);

bench_new_skewed!(u8: skewed_h_new_u08, 2000, 0.1f64);
bench_new_skewed!(u16: skewed_h_new_u16, 10000, 0.1f64);
bench_new_skewed!(u32: skewed_h_new_u32, 100000, 0.1f64);
bench_new_skewed!(u64: skewed_h_new_u64, 100000, 0.1f64);

bench_stdlib_skewed!(u8: skewed_h_stdlib_u08, 2000, 0.1f64);
bench_stdlib_skewed!(u16: skewed_h_stdlib_u16, 10000, 0.1f64);
bench_stdlib_skewed!(u32: skewed_h_stdlib_u32, 100000, 0.1f64);
bench_stdlib_skewed!(u64: skewed_h_stdlib_u64, 100000, 0.1f64);


macro_rules! bench_from_str_std {
	($t:ty: $name:ident, $samples:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let items: Vec<_> = (0..$samples).map(|_| format!("{}", rng.gen::<$t>()) ).collect();
			b.iter(|| {
				for str_ in &items {
					let x: Result<$t, _> = FromStr::from_str(str_);
					test::black_box(x);
				}
			});
		}
	};
}

macro_rules! bench_from_str_new {
	($t:ty: $name:ident, $samples:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let items: Vec<_> = (0..$samples).map(|_| format!("{}", rng.gen::<$t>()) ).collect();
			b.iter(|| {
				for str_ in &items {
					let x: Result<Wrapper<$t>, _> = FromStr::from_str(str_);
					test::black_box(x);
				}
			});
		}
	};
}


bench_from_str_std!(u8: bench_from_str_std_u08, 20);
bench_from_str_std!(u16: bench_from_str_std_u16, 20);
bench_from_str_std!(u32: bench_from_str_std_u32, 20);
bench_from_str_std!(u64: bench_from_str_std_u64, 20);
bench_from_str_std!(i8: bench_from_str_std_i08, 20);
bench_from_str_std!(i16: bench_from_str_std_i16, 20);
bench_from_str_std!(i32: bench_from_str_std_i32, 20);
bench_from_str_std!(i64: bench_from_str_std_i64, 20);

bench_from_str_new!(u8: bench_from_str_new_u08, 20);
bench_from_str_new!(u16: bench_from_str_new_u16, 20);
bench_from_str_new!(u32: bench_from_str_new_u32, 20);
bench_from_str_new!(u64: bench_from_str_new_u64, 20);
bench_from_str_new!(i8: bench_from_str_new_i08, 20);
bench_from_str_new!(i16: bench_from_str_new_i16, 20);
bench_from_str_new!(i32: bench_from_str_new_i32, 20);
bench_from_str_new!(i64: bench_from_str_new_i64, 20);
