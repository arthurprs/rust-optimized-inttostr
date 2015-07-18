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

macro_rules! bench_new_display {
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

macro_rules! bench_stdlib_display {
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

bench_new_display!(u8: display_l_new_u08, 2000, 5.0f64);
bench_new_display!(u16: display_l_new_u16, 10000, 5.0f64);
bench_new_display!(u32: display_l_new_u32, 100000, 5.0f64);
bench_new_display!(u64: display_l_new_u64, 100000, 5.0f64);

bench_stdlib_display!(u8: display_l_stdlib_u08, 2000, 5.0f64);
bench_stdlib_display!(u16: display_l_stdlib_u16, 10000, 5.0f64);
bench_stdlib_display!(u32: display_l_stdlib_u32, 100000, 5.0f64);
bench_stdlib_display!(u64: display_l_stdlib_u64, 100000, 5.0f64);

bench_new_display!(u8: display_m_new_u08, 2000, 0.9f64);
bench_new_display!(u16: display_m_new_u16, 10000, 0.9f64);
bench_new_display!(u32: display_m_new_u32, 100000, 0.9f64);
bench_new_display!(u64: display_m_new_u64, 100000, 0.9f64);

bench_stdlib_display!(u8: display_m_stdlib_u08, 2000, 0.9f64);
bench_stdlib_display!(u16: display_m_stdlib_u16, 10000, 0.9f64);
bench_stdlib_display!(u32: display_m_stdlib_u32, 100000, 0.9f64);
bench_stdlib_display!(u64: display_m_stdlib_u64, 100000, 0.9f64);

bench_new_display!(u8: display_h_new_u08, 2000, 0.1f64);
bench_new_display!(u16: display_h_new_u16, 10000, 0.1f64);
bench_new_display!(u32: display_h_new_u32, 100000, 0.1f64);
bench_new_display!(u64: display_h_new_u64, 100000, 0.1f64);

bench_stdlib_display!(u8: display_h_stdlib_u08, 2000, 0.1f64);
bench_stdlib_display!(u16: display_h_stdlib_u16, 10000, 0.1f64);
bench_stdlib_display!(u32: display_h_stdlib_u32, 100000, 0.1f64);
bench_stdlib_display!(u64: display_h_stdlib_u64, 100000, 0.1f64);


macro_rules! bench_stdlib_from_str {
    ($t:ty: $name:ident, $samples:expr, $skew:expr) => {
        #[bench]
        fn $name(b: &mut test::Bencher) {
            let mut rng: StdRng = SeedableRng::from_seed(SEED);
            let items: Vec<String> = (0..$samples).map(|_| {
                let x: f64 = rng.gen();
                let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(0.2f64);
                let x = (x * (<$t>::max_value() as f64).log10()) as u32;
                let x = 10u64.pow(x as u32);
                let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
                format!("{}", x as $t)
            }).collect();
            b.iter(|| {
                for str_ in &items {
                    let x: Result<$t, _> = FromStr::from_str(str_);
                    test::black_box(x);
                }
            });
        }
    };
}

macro_rules! bench_new_from_str {
    ($t:ty: $name:ident, $samples:expr, $skew:expr) => {
        #[bench]
        fn $name(b: &mut test::Bencher) {
            let mut rng: StdRng = SeedableRng::from_seed(SEED);
            let items: Vec<String> = (0..$samples).map(|_| {
                let x: f64 = rng.gen();
                let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(0.2f64);
                let x = (x * (<$t>::max_value() as f64).log10()) as u32;
                let x = 10u64.pow(x as u32);
                let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
                format!("{}", x as $t)
            }).collect();
            b.iter(|| {
                for str_ in &items {
                    let x: Result<Wrapper<$t>, _> = FromStr::from_str(str_);
                    test::black_box(x);
                }
            });
        }
    };
}


bench_new_from_str!(u8: from_str_l_new_u08, 2000, 5.0f64);
bench_new_from_str!(u16: from_str_l_new_u16, 10000, 5.0f64);
bench_new_from_str!(u32: from_str_l_new_u32, 100000, 5.0f64);
bench_new_from_str!(u64: from_str_l_new_u64, 100000, 5.0f64);

bench_stdlib_from_str!(u8: from_str_l_stdlib_u08, 2000, 5.0f64);
bench_stdlib_from_str!(u16: from_str_l_stdlib_u16, 10000, 5.0f64);
bench_stdlib_from_str!(u32: from_str_l_stdlib_u32, 100000, 5.0f64);
bench_stdlib_from_str!(u64: from_str_l_stdlib_u64, 100000, 5.0f64);

bench_new_from_str!(u8: from_str_m_new_u08, 2000, 0.9f64);
bench_new_from_str!(u16: from_str_m_new_u16, 10000, 0.9f64);
bench_new_from_str!(u32: from_str_m_new_u32, 100000, 0.9f64);
bench_new_from_str!(u64: from_str_m_new_u64, 100000, 0.9f64);

bench_stdlib_from_str!(u8: from_str_m_stdlib_u08, 2000, 0.9f64);
bench_stdlib_from_str!(u16: from_str_m_stdlib_u16, 10000, 0.9f64);
bench_stdlib_from_str!(u32: from_str_m_stdlib_u32, 100000, 0.9f64);
bench_stdlib_from_str!(u64: from_str_m_stdlib_u64, 100000, 0.9f64);

bench_new_from_str!(u8: from_str_h_new_u08, 2000, 0.1f64);
bench_new_from_str!(u16: from_str_h_new_u16, 10000, 0.1f64);
bench_new_from_str!(u32: from_str_h_new_u32, 100000, 0.1f64);
bench_new_from_str!(u64: from_str_h_new_u64, 100000, 0.1f64);

bench_stdlib_from_str!(u8: from_str_h_stdlib_u08, 2000, 0.1f64);
bench_stdlib_from_str!(u16: from_str_h_stdlib_u16, 10000, 0.1f64);
bench_stdlib_from_str!(u32: from_str_h_stdlib_u32, 100000, 0.1f64);
bench_stdlib_from_str!(u64: from_str_h_stdlib_u64, 100000, 0.1f64);