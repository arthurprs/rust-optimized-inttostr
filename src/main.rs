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
