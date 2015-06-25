#![feature(custom_derive)]
#![feature(test)]
#[cfg(test)] extern crate test;
// #[cfg(test)] extern crate strconv;
extern crate rand;

mod wrapper;
pub use wrapper::Wrapper;

fn collect_samples(samples: usize, skew: f64) -> Vec<u32> {
	use rand::{Rng, SeedableRng, StdRng};
	const SEED: &'static [usize] = &[0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

	let mut rng: StdRng = SeedableRng::from_seed(SEED);

	(0..samples).map(|_|{
		let x: f64 = rng.gen();
		let x = (1f64 + ((x - 1f64) * (x + 1f64))).powf(skew);
		let x = (x * (u32::max_value() as f64).log10()) as u32;
		let x = 10u64.pow(x as u32);
		let x = x + (rng.gen::<f64>() * x as f64 / 10f64) as u64;
		x as u32
	}).collect()
}

#[cfg(not(test))]
fn main() {
	let mut hist = [0u32; 11];
	for x in collect_samples(10000, 0.1f64) {
		hist[x.to_string().len() as usize] += 1;
	}
	println!("h: {:?}", hist);

	let mut hist = [0u32; 11];
	for x in collect_samples(10000, 0.6f64) {
		hist[x.to_string().len() as usize] += 1;
	}
	println!("m: {:?}", hist);

	let mut hist = [0u32; 11];
	for x in collect_samples(10000, 1.5f64) {
		hist[x.to_string().len() as usize] += 1;
	}
	println!("l: {:?}", hist);
}

#[cfg(test)]
pub mod bench;

#[cfg(test)]
mod tests {
	use super::Wrapper;
	use rand::{self, Rng};

	#[test]
	fn test_pos() {
		for i in (0i64..10000) {
			let n = Wrapper(i);
			assert_eq!(format!("{}", i), format!("{}", n));
		}
	}

	#[test]
	fn test_neg() {
		for i in (0i64..10000) {
			let n = Wrapper(-i);
			assert_eq!(format!("{}", -i), format!("{}", n));
		}
	}

	#[test]
	fn test_overflow() {
		assert_eq!("-128", format!("{}", Wrapper(-128i8)));
		assert_eq!("-32768", format!("{}", Wrapper(-32768i16)));
		assert_eq!("-2147483648", format!("{}", Wrapper(-2147483648i32)));
		assert_eq!("-9223372036854775808", format!("{}", Wrapper(-9223372036854775808i64)));
	}

	macro_rules! test_type {
		($t:ty: $name:ident) => {
			#[test]
			fn $name() {
				let mut rnd = rand::weak_rng();
				for _ in (0..1000000) {
					let i: $t = rnd.gen();
					assert_eq!(format!("{}", i), format!("{}", Wrapper(i)));
				}
			}
		}
	}

	test_type!(u8: test_u8);
	test_type!(u16: test_u16);
	test_type!(u32: test_u32);
	test_type!(u64: test_u64);
	test_type!(i8: test_i8);
	test_type!(i16: test_i16);
	test_type!(i32: test_i32);
	test_type!(i64: test_i64);
}