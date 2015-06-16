

use strconv::int2dec::UintToDec;
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
	($t:ty: $name:ident) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<_> = (0..1000).map(|_| Wrapper{n: rng.gen::<$t>()}).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
}

macro_rules! bench_strconv {
	($t:ty: $name:ident) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<_> = (0..1000).map(|_| UintToDec(rng.gen::<$t>())).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
}

macro_rules! bench_stdlib {
	($t:ty: $name:ident) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut rng: StdRng = SeedableRng::from_seed(SEED);
			let mut buf: VoidWriter = VoidWriter;
			let items: Vec<$t> = (0..1000).map(|_| rng.gen::<$t>()).collect();
			b.iter(|| {
				for i in &items {
					test::black_box(buf.write_fmt(format_args!("{}", *i)));
				}
			});
		}
	};
}

bench_new!(u8: new_u08);
bench_new!(u16: new_u16);
bench_new!(u32: new_u32);
bench_new!(u64: new_u64);

bench_strconv!(u8: strconv_u08);
bench_strconv!(u16: strconv_u16);
bench_strconv!(u32: strconv_u32);
bench_strconv!(u64: strconv_u64);

bench_strconv!(u8: stdlib_u08);
bench_strconv!(u16: stdlib_u16);
bench_strconv!(u32: stdlib_u32);
bench_strconv!(u64: stdlib_u64);
